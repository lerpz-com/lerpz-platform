use axum::extract::FromRef;
use sqlx::{Pool, Postgres};

#[derive(Clone, Debug)]
pub struct AppState {
    pub database: sqlx::PgPool,
}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Pool<Postgres> {
        state.database.clone()
    }
}
