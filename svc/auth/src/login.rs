//! Endpoint for user login.

use std::path::PathBuf;

use crate::{config::CONFIG, state::AppState};

use axum_extra::extract::{CookieJar, cookie::Cookie};
use lerpz_axum::error::{HandlerError, HandlerResult};
use lerpz_pwd::validate_pwd;

use axum::{
    Json,
    body::Body,
    extract::State,
    http::{Response, StatusCode, header},
};
use serde::Deserialize;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[axum::debug_handler]
pub async fn get() -> HandlerResult<Response<Body>> {
    let full_path = PathBuf::from(&CONFIG.OAUTH_ASSETS_PATH).join("authorize.html");

    let file = File::open(&full_path).await?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let content_type = "text/html; charset=utf-8";

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, content_type)
        .body(body)?)
}

#[axum::debug_handler]
pub async fn post(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> HandlerResult<CookieJar> {
    let mut database = state.database.acquire().await?;

    let user = sqlx::query_as!(
        lerpz_model::User,
        "SELECT * FROM users WHERE username = $1",
        body.username
    )
    .fetch_one(&mut *database)
    .await?;

    if !validate_pwd(&user.password_hash, &user.password_salt, &body.password).await? {
        return Err(HandlerError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid password",
            "The provided password is incorrect.",
        ));
    }

    let session_cookie = Cookie::build(("session", "secret_session_token"))
        .path("/")
        .secure(true);

    Ok(jar.add(session_cookie))
}
