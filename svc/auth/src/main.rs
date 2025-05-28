use std::net::Ipv4Addr;

use lerpz_auth::{error::HandlerResult, shutdown::shutdown_signal};

use axum::Router;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    if dotenvy::dotenv().is_err() {
        tracing::warn!("no .env file found");
    }

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::from(format!("{}=debug,info", env!("CARGO_CRATE_NAME")))
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/api/v1", axum::routing::get(endpoint));

    let addr = std::net::SocketAddr::from((Ipv4Addr::UNSPECIFIED, 4000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("server started listening on {addr}");

    let service = app.into_make_service();
    axum::serve(listener, service)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

#[axum::debug_handler]
async fn endpoint() -> HandlerResult<()> {
    "abc".parse::<u32>()?;
    Ok(())
}
