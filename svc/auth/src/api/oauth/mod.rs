mod authorize;
mod introspect;
mod revoke;
mod token;
mod userinfo;

use axum::routing::{get, post};

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/authorize", get(authorize::handler))
        .route("/token", post(token::handler))
        .route("/userinfo", get(userinfo::handler))
        .route("/introspect", post(introspect::handler))
        .route("/revoke", post(revoke::handler))
}
