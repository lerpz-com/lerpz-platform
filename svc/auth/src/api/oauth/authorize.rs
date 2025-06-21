use lerpz_utils::axum::error::{HandlerError, HandlerResult};

use axum::{http::StatusCode, response::Redirect, Form};
use serde::{Deserialize, Serialize};
use serde_urlencoded::to_string;
use url::Url;

/// Represents an OAuth 2.0 request to the authorization endpoint.
#[derive(Deserialize, Debug)]
#[serde(tag = "response_type")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AuthorizationRequest {
    AuthorizationCode(AuthorizationCodeRequest),
}

/// Represents an OAuth 2.0 response from the authorization endpoint.
pub enum AuthorizationResponse {
    AuthorizationCode(AuthorizationCodeResponse),
}

/// A request to initiate the OAuth 2.0 authorization code flow.
///
/// Source: https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.1
#[derive(Deserialize, Debug)]
pub struct AuthorizationCodeRequest {
    client_id: String,
    redirect_uri: String,
    scope: Option<String>,
    state: Option<String>,
}

/// A response to an authorization code request.
///
/// Source: https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2
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

/// A request to initiate the OAuth 2.0 implicit grant flow
///
/// Source: https://datatracker.ietf.org/doc/html/rfc6749#section-4.2.1
#[derive(Deserialize, Debug)]
pub struct ImplicitGrantRequest {
    client_id: String,
    redirect_uri: Option<String>,
    scope: Option<String>,
    state: Option<String>,
}

/// A response to an authorization request in the implicit grant flow.
///
/// Source: https://datatracker.ietf.org/doc/html/rfc6749#section-4.2.2
#[derive(Serialize, Debug)]
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
/// Source: https://datatracker.ietf.org/doc/html/rfc6749#section-4.2.2.1
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
pub async fn handler(Form(query): Form<AuthorizationRequest>) -> HandlerResult<Redirect> {
    let AuthorizationRequest::AuthorizationCode(req) = query;
    let res = authorization_code(&req)?;

    let redirect_uri = extend_url_query(&req.redirect_uri, &res)?;

    Ok(Redirect::to(redirect_uri.as_str()))
}

fn authorization_code(req: &AuthorizationCodeRequest) -> HandlerResult<AuthorizationCodeResponse> {
    Ok(AuthorizationCodeResponse::Success {
        code: Some("generated_code".to_string()),
        state: req.state.clone(),
    })
}

fn implicit_grant(req: ImplicitGrantRequest) -> HandlerResult<ImplicitGratResponse> {
    Ok(ImplicitGratResponse::Success {
        access_token: "generated_access_token".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: Some(3600),
        scope: req.scope,
        state: req.state,
    })
}

fn extend_url_query<T: Serialize>(url_str: &str, params: &T) -> Result<Url, HandlerError> {
    let mut url = Url::parse(url_str)
        .map_err(|_| HandlerError::new(
            StatusCode::BAD_REQUEST,
            "Invalid redirect URI",
            "The redirect URI provided is not valid."
        ))?;
    
    let query_string = serde_urlencoded::to_string(params)?;
    
    if !query_string.is_empty() {
        if url.query().is_some() {
            let existing_query = url.query().unwrap();
            url.set_query(Some(&format!("{}&{}", existing_query, query_string)));
        } else {
            url.set_query(Some(&query_string));
        }
    }
    
    Ok(url)
}
