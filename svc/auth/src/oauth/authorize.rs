//! Endpoint for OAuth 2.0 authorization.
//!
//! This only implements the Authorization Code + PKCE flow as per RFC 6749 and
//! RFC 7636. The implicit grant is deprecated and not implemented.

use std::path::PathBuf;

use lerpz_axum::error::{HandlerError, HandlerResult};

use axum::{
    body::Body,
    extract::{Query, State},
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::{config::CONFIG, state::AppState};

/// Represents an OAuth 2.0 request to the authorization endpoint.
#[derive(Deserialize, Debug)]
#[serde(tag = "response_type")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AuthorizationRequest {
    AuthorizationCode(AuthorizationCodeRequest),
}

/// Represents an OAuth 2.0 response from the authorization endpoint.
#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AuthorizationResponse {
    #[serde(rename = "code")]
    AuthorizationCode(AuthorizationCodeResponse),
}

/// A request to initiate the OAuth 2.0 authorization code flow.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.1
///
#[derive(Deserialize, Debug)]
#[serde(tag = "response_type")]
pub struct AuthorizationCodeRequest {
    client_id: Uuid,
    redirect_uri: String,
    scope: Option<String>,
    state: Option<String>,
}

/// A response to an authorization code request.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
#[derive(Serialize, Debug)]
pub enum AuthorizationCodeResponse {
    Success {
        code: Option<String>,
        state: Option<String>,
    },
    Failed {
        error: AuthorizationErrorKind,
        error_description: Option<String>,
        error_uri: Option<String>,
        state: Option<String>,
    },
}

/// Error kinds that the authorization endpoint might return.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationErrorKind {
    InvalidRequest,
    UnauthorizedClient,
    AccessDenied,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
    TemporarilyUnavailable,
}

#[axum::debug_handler]
pub async fn get(
    State(state): State<AppState>,
    Query(url): Query<AuthorizationCodeRequest>,
) -> HandlerResult<impl IntoResponse> {
    let mut database = state.database.acquire().await?;

    let oauth_client = sqlx::query_as!(
        lerpz_model::OAuthClient,
        "SELECT * FROM oauth_clients WHERE id = $1",
        &url.client_id
    )
    .fetch_one(&mut *database)
    .await?;

    let redirect_url = sqlx::query_as!(
        lerpz_model::RedirectUri,
        "SELECT * FROM redirect_uris WHERE uri = $1",
        &url.redirect_uri
    )
    .fetch_one(&mut *database)
    .await?;

    if oauth_client.id != redirect_url.client_id {
        return Err(HandlerError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid redirect URI",
            "The client ID does not match the redirect URI",
        ));
    }

    let full_path = PathBuf::from(&CONFIG.OAUTH_ASSETS_PATH).join("authorize.html");

    let file = File::open(&full_path).await?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let content_type = "text/html; charset=utf-8";

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, content_type)
        .body(body)?)
}
