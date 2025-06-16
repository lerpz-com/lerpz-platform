use crate::state::AppState;

use lerpz_utils::{
    axum::error::{HandlerError, HandlerResult},
    pwd::hash_pwd,
};

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

#[axum::debug_handler]
pub async fn handler(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> HandlerResult<()> {
    let mut db = state.database.acquire().await?;

    let password_salt = Uuid::new_v4().to_string();
    let password_hash = hash_pwd(body.password, &password_salt).await?;

    sqlx::query!(
        "INSERT INTO users (
        username,
        primary_email,
        password_hash,
        password_salt
        ) VALUES ($1, $2, $3, $4)",
        body.username,
        body.email,
        password_hash,
        password_salt
    )
    .execute(&mut *db)
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
