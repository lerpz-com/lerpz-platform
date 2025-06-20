mod authorize;
mod revoke;
mod token;
mod userinfo;

use crate::AppState;

use axum::routing::{get, post};

pub fn router(state: AppState) -> axum::Router<AppState> {
    axum::Router::<AppState>::new()
        .route(
            "/authorize",
            get(authorize::post_handler).post(authorize::get_handler),
        )
        .route("/revoke", post(revoke::handler))
        .route("/token", post(token::handler))
        .route("/userinfo", get(userinfo::handler))
        .with_state(state)
}
