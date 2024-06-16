use anyhow::Result;
use std::sync::Arc;

use childcare_app::driver::{
    modules::Modules,
    routes::startup,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let modules = Modules::new().await;

    startup::run(Arc::new(modules)).await.expect("run child care error");

    Ok(())
}
