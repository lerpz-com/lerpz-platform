pub mod config;
pub mod error;
pub mod api;
pub mod shutdown;

#[derive(Clone, Debug)]
pub struct AppState {
    pg: sqlx::PgPool
}
