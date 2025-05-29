//! This module defines the API endpoints for the authentication service.
//!
//! Follows RFC 6749 (OAuth 2.0) and RFC 6750 (Bearer Token Usage).
//!
//! # Auth Service Endpoints
//!
//! New User Journey:
//! 1. POST /register → Create account
//! 2. GET /verify-email → Verify email
//! 3. GET /oauth/authorize → Login & authorize app
//! 4. POST /oauth/token → Get access token
//!
//! Existing User Journey:
//! 1. GET /oauth/authorize → Login & authorize app  
//! 2. POST /oauth/token → Get access token
//!
//! Password Recovery:
//! 1. POST /forgot-password → Request reset
//! 2. POST /reset-password → Set new password
//!
//! Account Management:
//! 1. GET /verify-email → Verify email

mod oauth;
mod pwd_forgot;
mod pwd_reset;
mod register;
mod verify_email;

use crate::AppState;

pub fn router(state: AppState) -> axum::Router {
    axum::Router::<AppState>::new()
        .nest("/oauth", oauth::router(state.clone()))
        .route("/register", axum::routing::post(register::handler))
        .route("/forgot-password", axum::routing::post(pwd_forgot::handler))
        .route("/reset-password", axum::routing::post(pwd_reset::handler))
        .route("/verify-email", axum::routing::get(verify_email::handler))
        .with_state(state)
}
