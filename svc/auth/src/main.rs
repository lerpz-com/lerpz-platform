use lerpz_auth::{config::CONFIG, shutdown::shutdown_signal};

use axum::Router;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::from(format!("{}=debug,info", env!("CARGO_CRATE_NAME")))
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    #[cfg(debug_assertions)]
    if let Err(err) = dotenvy::dotenv() {
        tracing::warn!("no .env file found: {}", err);
    }

    let app = Router::new().nest("/api", lerpz_auth::api::router());

    let listener = tokio::net::TcpListener::bind(&CONFIG.ADDR).await?;
    tracing::info!("server started listening on {}", CONFIG.ADDR);

    let service = app.into_make_service();
    axum::serve(listener, service)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
