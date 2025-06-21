use lerpz_utils::axum::error::HandlerResult;

use axum::Form;
use serde::Deserialize;

/// A request to initiate the OAuth 2.0 authorization code flow.
///
/// https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.1
#[derive(Deserialize, Debug)]
pub struct AuthorizationCodeRequest {
    code: String,
    redirect_uri: String,
    client_id: String,
    scope: Option<String>,
    state: Option<String>,
}

/// A response to an authorization code request.
///
/// https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2
#[derive(Deserialize, Debug)]
pub enum AuthorizationCodeResponse {
    Success {
        code: Option<String>,
        state: String,
    },
    Failed {
        error: AuthorizationErrorKind,
        error_description: Option<String>,
        error_uri: Option<String>,
        state: String,
    },
}

/// A request to initiate the OAuth 2.0 implicit grant flow
///
/// https://datatracker.ietf.org/doc/html/rfc6749#section-4.2.1
pub struct ImplicitGrantRequest {
    response_type: String,
    client_id: String,
    redirect_uri: Option<String>,
    scope: Option<String>,
    state: Option<String>,
}

/// A response to an authorization request in the implicit grant flow.
///
/// https://datatracker.ietf.org/doc/html/rfc6749#section-4.2.2
pub enum ImplicitGratResponse {
    Success {
        access_token: String,
        token_type: String,
        expires_in: Option<u64>,
        scope: Option<String>,
        state: Option<String>,
    },
    Failed {
        error: AuthorizationErrorKind,
        error_description: Option<String>,
        error_uri: Option<String>,
        state: Option<String>,
    },
}

/// Possible errors that can occur during the implicit grant flow.
/// 
/// https://datatracker.ietf.org/doc/html/rfc6749#section-4.2.2.1
#[derive(Deserialize, Debug)]
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
pub async fn get_handler(Form(query): Form<AuthorizationCodeRequest>) -> HandlerResult<()> {
    todo!()
}

#[axum::debug_handler]
pub async fn post_handler() -> HandlerResult<()> {
    todo!()
}
