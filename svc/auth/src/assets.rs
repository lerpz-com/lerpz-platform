//! Endpoint for OAuth 2.0 authorization.
//!
//! This endpoint returns assets like HTML, CSS, and JavaScript files.

use std::path::PathBuf;

use axum::{
    body::Body,
    extract::Path,
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use lerpz_axum::error::{HandlerError, HandlerResult};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::config::CONFIG;

#[axum::debug_handler]
pub async fn get(Path(file_path): Path<String>) -> HandlerResult<impl IntoResponse> {
    let full_path = PathBuf::from(&CONFIG.OAUTH_ASSETS_PATH).join(file_path);

    if !full_path.starts_with(&CONFIG.OAUTH_ASSETS_PATH) {
        return Err(HandlerError::new(
            StatusCode::FORBIDDEN,
            "Forbidden",
            "Access to the requested file is forbidden.",
        ));
    }

    let file = File::open(&full_path).await.map_err(|_| {
        HandlerError::new(
            StatusCode::NOT_FOUND,
            "File not found",
            "The requested file could not be found.",
        )
    })?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let content_type = mime_guess::from_path(&full_path)
        .first_or_octet_stream()
        .to_string();

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, content_type)
        .body(body)
        .unwrap())
}
