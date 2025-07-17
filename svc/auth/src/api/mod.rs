//! This module defines the API endpoints for a authentication service.
//! 
//! The auth service implements the OAuth 2.0 framework for user authentication
//! and authorization. The service prepares for the future release of OAuth 2.1
//! which means that deprecated endpoints want be implemented.
//! 
//! # Auth Service Endpoints
//! 
//! New User Journey:
//! 1. POST /register → Create account
//! 2. GET /verify-email → Verify email
//! 
//! Existing User Journey:
//! 1. GET /oauth/authorize → Login & authorize app  
//! 2. POST /oauth/token → Get access token
//! 
//! Password Recovery:
//! 1. POST /forgot-password → Request reset
//! 2. POST /reset-password → Set new password

mod email_verify;
mod login;
mod oauth;
mod pwd_forgot;
mod pwd_reset;
mod register;

use axum::routing::{get, post};

use crate::AppState;

pub fn router(state: AppState) -> axum::Router {
    axum::Router::<AppState>::new()
        .nest("/oauth", oauth::router(state.clone()))
        .route("/login", post(login::handler))
        .route("/register", post(register::handler))
        .route("/verify-email", get(email_verify::handler))
        .route("/forgot-password", post(pwd_forgot::handler))
        .route("/reset-password", post(pwd_reset::handler))
        .with_state(state)
}
