use axum::{Json, http::StatusCode};
use lerpz_utils::axum::{
    error::{HandlerError, HandlerResult},
    middelware::db::Database,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct RegisterRequest {
    email: String,
    username: String,
}

#[axum::debug_handler]
pub async fn handler(Database(mut conn): Database, Json(body): Json<RegisterRequest>) -> HandlerResult<()> {
    sqlx::query_as!(
        lerpz_core::db::User,
        "INSERT INTO users (email, username) VALUES ($1, $2)",
        body.email,
        body.username,
    )
    .fetch_one(&mut *conn)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(db_err) => match db_err.kind() {
            sqlx::error::ErrorKind::UniqueViolation => HandlerError::new(
                StatusCode::CONFLICT,
                "Unique violation",
                "Email or username already exsits",
            ),
            _ => HandlerError::from(db_err),
        },
        _ => HandlerError::from(err),
    })?;

    Ok(())
}
