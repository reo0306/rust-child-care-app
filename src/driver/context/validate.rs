use async_trait::async_trait;
use axum::{
    extract::{
        rejection::JsonRejection, FromRequest, Request,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
use validator::Validate;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedRequest<T>(pub T);

#[derive(Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
}

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedRequest<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;

        value.validate()?;

        Ok(ValidatedRequest(value))
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Validation Error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::JsonRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}