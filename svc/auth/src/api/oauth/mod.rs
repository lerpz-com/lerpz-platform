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

mod authorize;
mod revoke;
mod token;
mod userinfo;

/// Possible errors the authorization server might return.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
pub fn router(state: AppState) -> axum::Router<AppState> {
    axum::Router::<AppState>::new()
        .route("/authorize", get(authorize::handler))
        .route("/token", post(token::handler))
        .route("/revoke", post(revoke::handler))
        .route("/userinfo", get(userinfo::handler))
        .with_state(state)
}
