//! Endpoint for creating tokens.

use crate::state::AppState;

use lerpz_axum::{
    error::{HandlerError, HandlerResult},
    middleware::validate::Validated,
};

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug)]
#[serde(tag = "grant_type", rename_all = "snake_case")]
pub enum TokenRequest {
    AuthorizationCode(AuthorizationCode),
}

#[derive(Deserialize, Debug)]
pub struct AuthorizationCode {
    pub code: String,
}

#[derive(Serialize, Debug, Validate)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_at: u64,
}

#[axum::debug_handler]
pub async fn post(
    State(state): State<AppState>,
    Json(body): Json<TokenRequest>,
) -> HandlerResult<Json<AccessToken>> {
    let mut database = state.database.acquire().await?;

    let refresh_token = sqlx::query_as!(
        lerpz_model::RefreshTokens,
        "SELECT * FROM refresh_tokens WHERE token = $1",
        body.refresh_token
    )
    .fetch_optional(&mut *database)
    .await?;

    let refresh_token = refresh_token.ok_or_else(HandlerError::unauthorized)?;

    Ok(Json(AccessToken {
        access_token: "new_access_token".to_string(),
        token_type: "Bearer",
        expires_at: 3600,
    }))
}
