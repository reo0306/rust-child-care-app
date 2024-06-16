use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Extension,
    Json
};
use std::sync::Arc;
use tracing::error;

use crate::driver::{
    context::validate::ValidatedRequest,
    model::baby::{JsonBabyView, JsonCreateBaby, JsonUpdateBaby},
    modules::{Modules, ModulesExt},
};

#[tracing::instrument(skip(modules))]
pub async fn find_baby(
    Extension(modules): Extension<Arc<Modules>>,
    Path(id): Path<String>,
    ) -> Result<impl IntoResponse, StatusCode> {
        let res = modules.baby_use_case().find_baby(id).await;

        match res {
            Ok(baby) => baby.map(|b| {
                let json: JsonBabyView = b.into();

                (StatusCode::OK, Json(json))
            })
            .ok_or_else(|| StatusCode::NOT_FOUND),
            Err(err) => {
                error!("Find baby error: {:?}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
}

#[tracing::instrument(skip(modules))]
pub async fn create_baby(
    Extension(modules): Extension<Arc<Modules>>,
    ValidatedRequest(params ): ValidatedRequest<JsonCreateBaby>,
    ) -> Result<impl IntoResponse, StatusCode> {
        modules
            .baby_use_case()
            .create_baby(params.into())
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(|err| {
                error!("Create baby error: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })
}

#[tracing::instrument(skip(modules))]
pub async fn update_baby(
    Extension(modules): Extension<Arc<Modules>>,
    Path(id): Path<String>,
    Json(params): Json<JsonUpdateBaby>,
    ) -> Result<impl IntoResponse, StatusCode> {
        modules
            .baby_use_case()
            .update_baby(id, params.into())
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(|err| {
                error!("Update baby error: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })
}

#[tracing::instrument(skip(modules))]
pub async fn delete_baby(
    Extension(modules): Extension<Arc<Modules>>,
    Path(id): Path<String>,
    ) -> Result<impl IntoResponse, StatusCode> {
        modules
            .baby_use_case()
            .delete_baby(id)
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(|err| {
                error!("Delete baby error: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })
}
