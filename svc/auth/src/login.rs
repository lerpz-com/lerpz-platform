//! Endpoint for user login.

use crate::state::AppState;

use lerpz_axum::error::{HandlerError, HandlerResult};
use lerpz_pwd::validate_pwd;

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    username: String,
    password: String,
    client_id: String,
}

#[axum::debug_handler]
pub async fn post(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
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
