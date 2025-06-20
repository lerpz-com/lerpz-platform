use lerpz_utils::axum::error::HandlerResult;

#[axum::debug_handler]
pub async fn get_handler() -> HandlerResult<()> {
    todo!()
}

#[axum::debug_handler]
pub async fn post_handler() -> HandlerResult<()> {
    todo!()
}
