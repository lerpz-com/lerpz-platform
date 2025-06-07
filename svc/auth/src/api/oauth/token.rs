use axum::{Json, http::StatusCode};
use lerpz_utils::axum::error::{HandlerError, HandlerResult};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TokenGrantType {
    AuthorizationCode(),
    ClientCredentials(),
    Password(PasswordGrant),
    RefreshToken(String),
    Unknown
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordGrant {
    pub username: String,
    pub password: String,
}

#[axum::debug_handler]
pub async fn handler(Json(body): Json<TokenGrantType>) -> HandlerResult<()> {
    use TokenGrantType::*;

    match body {
        AuthorizationCode() => authorization_code_handler().await,
        ClientCredentials() => Ok(()),
        Password(_) => Ok(()),
        RefreshToken(_) => Ok(()),
        Unknown => Err(HandlerError::new(
            StatusCode::BAD_REQUEST,
            "Unsupported grant type.",
            "The provided grant type is not supported.",
        ))
    }
}

pub async fn authorization_code_handler() -> HandlerResult<()> {
    Ok(())
}
