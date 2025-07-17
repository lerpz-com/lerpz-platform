//! Endpoint for user registration.

use crate::state::AppState;

use lerpz_axum::{
    error::{HandlerError, HandlerResult},
    middleware::validate::Validated,
};
use lerpz_pwd::hash_pwd;

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    email: String,
    #[validate(length(
        min = 3,
        max = 32,
        message = "Username must be between 3 and 32 characters"
    ))]
    username: String,
    #[validate(length(
        min = 8,
        max = 128,
        message = "Password must be between 8 and 128 characters"
    ))]
    password: String,
}

#[axum::debug_handler]
pub async fn handler(
    State(state): State<AppState>,
    Validated(Json(body)): Validated<Json<RegisterRequest>>,
) -> HandlerResult<()> {
    let mut database = state.database.acquire().await?;

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
    .execute(&mut *database)
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
