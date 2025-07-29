//! This is the implementation of the OAuth 2.0 framework (server).
//!
//! There is an in-progress effort to update the framework to OAuth 2.1. Flows
//! like the `implicit grant` and `password grant` are not implemented as they
//! are deprecated and will be removed in the future update of the framework.
//!
//! The goal is to also implement the OpenID Connect standard. This means that
//! the code will be written with that in mind.
//!
//! Sources:
//! - https://datatracker.ietf.org/doc/html/rfc6749
//! - https://datatracker.ietf.org/doc/html/rfc7636
//! - https://openid.net/specs/openid-connect-core-1_0.html

use crate::AppState;

use axum::routing::{get, post};

mod authorize;
mod revoke;
mod token;
mod userinfo;

/// This is the main router for the OAuth 2.0 API.
pub fn router(state: AppState) -> axum::Router<AppState> {
    axum::Router::<AppState>::new()
        .route("/authorize", get(authorize::get))
        .route("/revoke", post(revoke::post))
        .route("/token", post(token::post))
        .route("/userinfo", get(userinfo::get))
        .with_state(state)
}
