//! Endpoint for OAuth 2.0 authorization.
//!
//! This only implements the Authorization Code + PKCE flow as per RFC 6749 and
//! RFC 7636. The implicit grant is deprecated and not implemented.

use std::sync::LazyLock;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::Rng;
use serde::Deserialize;
use strum::AsRefStr;
use url::Url;
use uuid::Uuid;

use crate::{config::CONFIG, state::AppState};

static PROBLEM_URL: LazyLock<Url> =
    LazyLock::new(|| Url::parse(&format!("{}/problem", CONFIG.ORIGIN)).unwrap());

/// A request to initiate the OAuth 2.0 authorization code flow.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.1
#[derive(Deserialize, Debug)]
pub struct AuthorizationCodeRequest {
    response_type: String,
    client_id: Uuid,
    redirect_uri: String,
    scope: String,
    state: Option<String>,
}

/// A response to an authorization code request.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
type AuthorizationCodeResponse = Result<Success, Failed>;

/// Successful authorization response.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2
#[derive(Debug)]
pub struct Success {
    /// The url to redirect to.
    url: Url,
    /// The code given the client can use to optain a access and refresh token
    code: String,
    /// State that might have been given in the request.
    state: Option<String>,
}

impl Success {
    /// Generate a new sucess response
    pub fn new(url: Url, code: String, state: Option<String>) -> Self {
        Self { url, code, state }
    }
}

impl IntoResponse for Success {
    fn into_response(self) -> Response {
        let mut url = self.url;
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("error", &self.code);
        }
        Redirect::to(url.as_str()).into_response()
    }
}

/// Failed authorization response.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
#[derive(Debug)]
pub struct Failed {
    /// The url to redirect to.
    url: Url,
    /// What kind of error happend.
    error: ErrorKind,
    /// An optional description of the error.
    error_description: Option<String>,
    /// An optional URI to documentation
    error_uri: Option<String>,
    /// State that might have been given in the request.
    state: Option<String>,
}

/// Error kinds that the authorization endpoint might return.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
#[derive(AsRefStr, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum ErrorKind {
    InvalidRequest,
    UnauthorizedClient,
    AccessDenied,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
    TemporarilyUnavailable,
}

impl Failed {
    /// Generate a new sucess response
    pub fn new(url: Url, error: ErrorKind, state: Option<String>) -> Self {
        Self {
            url,
            error,
            state,
            error_description: None,
            error_uri: None,
        }
    }

    /// Adds a error description to the response.
    pub fn with_error_description(mut self, description: String) -> Self {
        self.error_description = Some(description);
        self
    }

    /// Adds the error URI to the response.
    pub fn with_error_uri(mut self, uri: String) -> Self {
        self.error_uri = Some(uri);
        self
    }
}

impl IntoResponse for Failed {
    fn into_response(self) -> Response {
        let mut url = self.url;
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("error", &self.error.as_ref());
        }
        Redirect::to(url.as_str()).into_response()
    }
}

impl<E> From<E> for Failed
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        todo!()
    }
}

#[axum::debug_handler]
pub async fn get(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(query): Query<AuthorizationCodeRequest>,
) -> AuthorizationCodeResponse {
    let mut database = state.database.acquire().await?;

    let oauth_client = sqlx::query_as!(
        lerpz_model::OAuthClient,
        "SELECT * FROM oauth_clients WHERE id = $1",
        &query.client_id
    )
    .fetch_optional(&mut *database)
    .await?;

    let redirect_uri = sqlx::query_as!(
        lerpz_model::RedirectUri,
        "SELECT * FROM redirect_uris WHERE uri = $1",
        &query.redirect_uri
    )
    .fetch_optional(&mut *database)
    .await?;

    let (_, redirect_uri) = match (oauth_client, redirect_uri) {
        (Some(oauth_client), Some(redirect_uri)) => (oauth_client, redirect_uri),
        _ => {
            return Err(Failed::new(
                PROBLEM_URL.clone(),
                ErrorKind::InvalidRequest,
                query.state,
            ));
        }
    };

    if query.response_type != "code".to_string() {
        todo!()
    }

    let _session = match jar.get("session") {
        Some(session) => session,
        None => return todo!(),
    };

    // TODO: Verify session token

    // TODO: Generate code

    // TODO: Don't use unwrap

    let response = Success {
        url: Url::parse(&redirect_uri.uri)?,
        code: generate_code_verifier(),
        state: query.state,
    };

    Ok(response)
}

/// Generate a random code verifier for PKCE.
///
/// This function generates a random 64-byte array and encodes it using
/// base64url encoding without padding. The resulting string is 86 characters
/// long, which meets RFC 7636 requirements (43-128 characters).
fn generate_code_verifier() -> String {
    let mut rng = rand::rng();
    let mut verifier_bytes = [0u8; 64]; // 64 bytes = 86 base64url chars
    rng.fill(&mut verifier_bytes);
    URL_SAFE_NO_PAD.encode(verifier_bytes)
}

/// Generate a login URL for the given authorization code request.
fn get_login_url(query: AuthorizationCodeRequest) -> Redirect {
    // TODO: Remove - this comment is only for debugging purposes.
    // http://localhost:3001/oauth/v2.0/authorize?client_id=cdd37e5a-a554-4535-bff2-45ba130b05b4&redirect_uri=http://localhost:3000&scope=openid.

    // TODO: Make sure the login URL is not hardcoded.
    let mut login_uri = Url::parse("http://localhost:3001/login").unwrap();

    {
        let mut pairs = login_uri.query_pairs_mut();
        pairs
            .append_pair("client_id", &query.client_id.to_string())
            .append_pair("redirect_uri", &query.redirect_uri)
            .append_pair("scope", &query.scope);

        if let Some(state) = query.state {
            pairs.append_pair("state", &state);
        }
    }

    Redirect::to(login_uri.as_str())
}
