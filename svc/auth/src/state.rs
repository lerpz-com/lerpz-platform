//! Axum state shared between endpoints.

use axum::extract::FromRef;
use bb8_redis::{RedisConnectionManager, bb8};
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub database: sqlx::PgPool,
    pub redis: bb8::Pool<RedisConnectionManager>,
}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Pool<Postgres> {
        state.database.clone()
    }
}

impl FromRef<AppState> for bb8::Pool<RedisConnectionManager> {
    fn from_ref(state: &AppState) -> Self {
        state.redis.clone()
    }
}
