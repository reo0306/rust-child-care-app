use anyhow::Result;
use std::sync::Arc;
use axum::{
    routing::{delete, get, patch, post}, Extension, Router
};

use crate::driver::{
    modules::Modules,
    routes::{
        baby::{
            find_baby, create_baby, update_baby, delete_baby,
        },
        childcare::{
            find_child_care, create_child_care, update_child_care, delete_child_care,
        }
    }
};

pub async fn run(modules: Arc<Modules>) -> Result<()> {
    let baby_router = Router::new()
        .route("/", post(create_baby))
        .route("/:id", get(find_baby))
        .route("/:id", patch(update_baby))
        .route("/:id", delete(delete_baby));

    let child_care_router = Router::new()
        .route("/", post(create_child_care))
        .route("/:id", get(find_child_care))
        .route("/:id", patch(update_child_care))
        .route("/:id", delete(delete_child_care));

    let app = Router::new()
        .nest("/baby", baby_router)
        .nest("/child-care", child_care_router)
        .layer(Extension(modules));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("Server cannnot launch!"));

    Ok(())
}