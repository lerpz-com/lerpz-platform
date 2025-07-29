//! Axum state shared between endpoints.

use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub database: sqlx::PgPool,
    pub redis: Arc<redis::Client>,
}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Pool<Postgres> {
        state.database.clone()
    }
}

impl FromRef<AppState> for Arc<redis::Client> {
    fn from_ref(state: &AppState) -> Self {
        state.redis.clone()
    }
}
