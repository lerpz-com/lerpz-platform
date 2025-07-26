//! Endpoint to handle password reset requests.

use lerpz_axum::error::HandlerResult;

#[axum::debug_handler]
pub async fn post() -> HandlerResult<()> {
    todo!()
}
