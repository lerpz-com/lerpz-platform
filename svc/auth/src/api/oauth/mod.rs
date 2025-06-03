mod authorize;
mod revoke;
mod token;
mod userinfo;

use crate::AppState;

use axum::routing::{get, post};

pub fn router(state: AppState) -> axum::Router<AppState> {
    axum::Router::<AppState>::new()
        .route("/authorize", get(authorize::handler))
        .route("/token", post(token::handler))
        .route("/userinfo", get(userinfo::handler))
        .route("/revoke", post(revoke::handler))
        .with_state(state)
}
