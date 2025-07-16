//! This is the implementation of the OAuth 2.0 authorization server.
//! 
//! The implicit grant flow is deprecated and not implemented in this version.
//! Instead the authorization code flow + PKCE is used, which is more secure and
//! recommended for public clients.
//! 
//! Sources:
//! - https://datatracker.ietf.org/doc/html/rfc6749
//! - https://datatracker.ietf.org/doc/html/rfc7636

use crate::AppState;

use axum::routing::{get, post};
use serde::Serialize;

mod authorize;
mod revoke;
mod token;
mod userinfo;

/// What the authorization server returns in case of a bad request.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
pub struct AuthorizationError {
    pub error: AuthorizationErrorKind,
    pub error_description: Option<String>,
    pub error_uri: Option<String>,
}

/// Possible errors the authorization server might return.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationErrorKind {
    InvalidRequest,
    UnauthorizedClient,
    AccessDenied,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
    TemporarilyUnavailable,
}

pub fn router(state: AppState) -> axum::Router<AppState> {
    axum::Router::<AppState>::new()
        .route("/authorize", get(authorize::handler))
        .route("/token", post(token::handler))
        .route("/revoke", post(revoke::handler))
        .route("/userinfo", get(userinfo::handler))
        .with_state(state)
}
