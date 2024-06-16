use axum::{
    http::StatusCode,
    response::IntoResponse,
    Extension,
    extract::Path,
    Json,
};
use std::sync::Arc;
use tracing::error;

use crate::driver::{
    context::validate::ValidatedRequest,
    model::childcare::{JsonChildCareView, JsonCreateChildCare, JsonUpdateChildCare},
    modules::{Modules, ModulesExt},
};

#[tracing::instrument(skip(modules))]
pub async fn find_child_care(
    Extension(modules): Extension<Arc<Modules>>,
    Path(id): Path<String>,
    ) -> Result<impl IntoResponse, StatusCode> {
        let res = modules
            .child_care_use_case()
            .find_child_care(id)
            .await;
        
        match res {
            Ok(cc) => cc.map(|c| {
                let json: JsonChildCareView = c.into();

                (StatusCode::OK, Json(json))
            })
            .ok_or_else(|| StatusCode::NOT_FOUND),
            Err(err) => {
                error!("find child care error: {:?}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
}

#[tracing::instrument(skip(modules))]
pub async fn create_child_care(
    Extension(modules): Extension<Arc<Modules>>,
    ValidatedRequest(params): ValidatedRequest<JsonCreateChildCare>,
    ) -> Result<impl IntoResponse, StatusCode> {
        modules
            .child_care_use_case()
            .create_child_care(params.into())
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(|err| {
                error!("Create child care error: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })
}

#[tracing::instrument(skip(modules))]
pub async fn update_child_care(
    Extension(modules): Extension<Arc<Modules>>,
    Path(id): Path<String>,
    Json(params): Json<JsonUpdateChildCare>,
    ) -> Result<impl IntoResponse, StatusCode> {
        modules
            .child_care_use_case()
            .update_child_care(id, params.into())
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(|err| {
                error!("Update child care error: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })
}

#[tracing::instrument(skip(modules))]
pub async fn delete_child_care(
    Extension(modules): Extension<Arc<Modules>>,
    Path(id): Path<String>,
    ) -> Result<impl IntoResponse, StatusCode> {
        modules
            .child_care_use_case()
            .delete_child_care(id)
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(|err| {
                error!("Delete child care error: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })
}