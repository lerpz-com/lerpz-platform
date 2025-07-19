//! This is the implementation of the OAuth 2.0 framework (server).
//! 
//! Also implements the identity layer provided by OpenID Connect Core 1.0.
//! 
//! There is an in-progress effort to update the framework to OAuth 2.1. Flows
//! like the `implicit grant` and `password grant` are not implemented as they
//! are deprecated and will be removed in the future update of the framework.
//! 
//! Sources:
//! - https://datatracker.ietf.org/doc/html/rfc6749
//! - https://datatracker.ietf.org/doc/html/rfc7636
//! - https://openid.net/specs/openid-connect-core-1_0.html

use crate::AppState;

use axum::routing::{get, post};

mod authorize;
mod revoke;
mod assets;
mod token;
mod userinfo;

/// This is the main router for the OAuth 2.0 API.
pub fn router(state: AppState) -> axum::Router<AppState> {
    axum::Router::<AppState>::new()
        .route("/assets/{*path}", get(assets::handler))
        .route("/authorize", get(authorize::handler))
        .route("/token", post(token::handler))
        .route("/revoke", post(revoke::handler))
        .route("/userinfo", get(userinfo::handler))
        .with_state(state)
}
