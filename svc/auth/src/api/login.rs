//! Endpoint for user login.

use crate::state::AppState;

use lerpz_axum::{
    error::{HandlerError, HandlerResult},
    middleware::validate::Validated,
};
use lerpz_pwd::validate_pwd;

use axum::{extract::State, http::StatusCode, Form};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RegisterRequest {
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
    Validated(Form(body)): Validated<Form<RegisterRequest>>,
) -> HandlerResult<()> {
    let mut database = state.database.acquire().await?;

    let user = sqlx::query_as!(
        lerpz_model::User,
        "SELECT * FROM users WHERE username = $1",
        body.username
    )
    .fetch_one(&mut *database)
    .await?;

    if !validate_pwd(&user.password_hash, &user.password_salt, &body.password).await? {
        return Err(HandlerError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid password",
            "The provided password is incorrect.",
        ));
    }

    Ok(())
}
