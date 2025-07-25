//! This module defines the API endpoints for a authentication service.

mod assets;
mod email_verify;
mod login;
mod pwd_forgot;
mod pwd_reset;
mod register;

use axum::routing::{get, post};

use crate::AppState;

pub fn router(state: AppState) -> axum::Router {
    axum::Router::<AppState>::new()
        .route("/register", post(register::handler))
        .route("/verify-email", get(email_verify::handler))
        .route("/login", post(login::handler))
        .route("/forgot-password", post(pwd_forgot::handler))
        .route("/reset-password", post(pwd_reset::handler))
        .route("/assets/{*path}", get(assets::handler))
        .with_state(state)
}
