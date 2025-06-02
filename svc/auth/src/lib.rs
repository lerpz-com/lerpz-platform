pub mod config;
pub mod api;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: sqlx::PgPool
}
