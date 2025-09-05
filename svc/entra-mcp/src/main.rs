use crate::config::CONFIG;
use crate::entra::Entra;

use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};
use tracing_subscriber::{self, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use axum::Router;
use lerpz_axum::shutdown_signal;

mod config;
mod entra;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::from(format!("{}=debug,info", env!("CARGO_CRATE_NAME")))
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    #[cfg(debug_assertions)]
    {
        use std::path::PathBuf;

        let env_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), ".env"]);
        if let Err(err) = dotenvy::from_path(&env_path) {
            tracing::warn!("failed loading .env file: {}", err);
        }
    }

    let service = StreamableHttpService::new(
        || Ok(Entra::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    let app = Router::new().nest_service("/mcp", service);

    let listener = tokio::net::TcpListener::bind(&CONFIG.ADDR).await?;
    tracing::info!("server started listening on {}", CONFIG.ADDR);

    let service = app.into_make_service();
    axum::serve(listener, service)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
