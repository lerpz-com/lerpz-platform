#![allow(unused)]
//! Endpoint for OAuth 2.0 authorization.
//!
//! This only implements the Authorization Code + PKCE flow as per RFC 6749 and
//! RFC 7636. The implicit grant is deprecated and not implemented.

use std::{
    path::PathBuf,
    sync::{Arc, LazyLock},
};

use lerpz_axum::error::{HandlerError, HandlerResult};

use axum::{
    body::Body, extract::Query, http::{header, Response, StatusCode}, response::IntoResponse
};
use serde::{Deserialize, Serialize};
use tinytemplate::TinyTemplate;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use url::Url;

use crate::config::CONFIG;

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
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "response_type")]
pub struct AuthorizationCodeRequest {
    client_id: String,
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
pub async fn handler(Query(url): Query<AuthorizationCodeRequest>) -> HandlerResult<impl IntoResponse> {
    let full_path = PathBuf::from(&CONFIG.OAUTH_ASSETS_PATH).join("authorize.html");

    let file = File::open(&full_path).await?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let content_type = "text/html; charset=utf-8";

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, content_type)
        .body(body)?)
}

fn authorization_code(req: &AuthorizationCodeRequest) -> HandlerResult<AuthorizationCodeResponse> {
    Ok(AuthorizationCodeResponse::Success {
        code: Some("generated_code".to_string()),
        state: req.state.clone(),
    })
}

fn extend_url_query<T: Serialize>(url_str: &str, params: &T) -> Result<Url, HandlerError> {
    let mut url = Url::parse(url_str).map_err(|_| {
        HandlerError::new(
            StatusCode::BAD_REQUEST,
            "Invalid redirect URI",
            "The redirect URI provided is not a valid URI.",
        )
    })?;

    let query_string = serde_urlencoded::to_string(params)?;
    if !query_string.is_empty() {
        if let Some(existing_query) = url.query() {
            url.set_query(Some(&format!("{}&{}", existing_query, query_string)));
        } else {
            url.set_query(Some(&query_string));
        }
    }

    Ok(url)
}
