//! Endpoint for user login.

use crate::{config::CONFIG, state::AppState};

use std::path::PathBuf;

use lerpz_axum::error::{HandlerError, HandlerResult};
use lerpz_pwd::validate_pwd;

use axum::{
    Json,
    body::Body,
    extract::State,
    http::{Response, StatusCode, header},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use rand::Rng;
use redis::AsyncCommands;
use serde::Deserialize;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub const SESSION_COOKIE_NAME: &str = "session";

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

    let session_token = generate_session_token();

    let mut redis = state.redis.get().await?;
    let redis_options = redis::SetOptions::default()
        .conditional_set(redis::ExistenceCheck::NX)
        .with_expiration(redis::SetExpiry::EX(3600));
    let _: () = redis
        .set_options(&session_token, user.id.to_string(), redis_options)
        .await?;

    let mut session_cookie = Cookie::build((SESSION_COOKIE_NAME, session_token))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict);

    #[cfg(debug_assertions)]
    {
        session_cookie = session_cookie.secure(false);
    }

    Ok(jar.add(session_cookie))
}

/// Generate a random session token.
///
/// The token will be a 32-byte hexadecimal string. This means a 64-character
/// string will be generated. The string is URL-safe since it uses hexadecimal
/// characters.
pub fn generate_session_token() -> String {
    let mut rng = rand::rng();
    let mut token = [0u8; 32];
    rng.fill(&mut token);
    hex::encode(token)
}
