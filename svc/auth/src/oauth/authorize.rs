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
use http::uri::{Builder as UriBuilder, Uri};
use rand::Rng;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use uuid::Uuid;

use crate::state::AppState;

static PROBLEM_URI: LazyLock<Uri> = LazyLock::new(|| "/problem".parse::<Uri>().unwrap());

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
#[derive(Serialize, Debug)]
pub struct Success {
    /// The uri to redirect to.
    #[serde(skip)]
    uri: Uri,
    /// The code given the client can use to optain a access and refresh token
    code: String,
    /// State that might have been given in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
}

impl Success {
    /// Generate a new sucess response
    pub fn new(uri: Uri, code: String, state: Option<String>) -> Self {
        Self { uri, code, state }
    }
}

impl IntoResponse for Success {
    fn into_response(self) -> Response {
        let query = serde_urlencoded::to_string(self);
        let builder = UriBuilder::from(self.uri);
        builder.path_and_query(query);
        Redirect::to(uri.as_str()).into_response()
    }
}

/// Failed authorization response.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
#[derive(Serialize, Debug)]
pub struct Failed {
    /// The uri to redirect to.
    #[serde(skip)]
    uri: Uri,
    /// What kind of error happend.
    error: ErrorKind,
    /// An optional description of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    error_description: Option<String>,
    /// An optional URI to documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    error_uri: Option<String>,
    /// State that might have been given in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
}

/// Error kinds that the authorization endpoint might return.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
#[derive(Serialize, AsRefStr, Debug)]
#[serde(rename_all = "snake_case")]
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
    pub fn new(uri: Uri, error: ErrorKind, state: Option<String>) -> Self {
        Self {
            uri,
            error,
            state,
            error_description: None,
            error_uri: None,
        }
    }

    /// Adds a error description to the response.
    pub fn with_error_description(mut self, description: impl Into<String>) -> Self {
        self.error_description = Some(description.into());
        self
    }

    /// Adds the error URI to the response.
    pub fn with_error_uri(mut self, uri: impl Into<String>) -> Self {
        self.error_uri = Some(uri.into());
        self
    }
}

impl IntoResponse for Failed {
    fn into_response(self) -> Response {
        let uri = self.uri.clone();
        let query = serde_urlencoded::to_string(self).unwrap();
        let builder = UriBuilder::from(uri)
            .path_and_query(format!("{}?{}", path, query));

        let uri = builder.build().unwrap();

        Redirect::to(&uri.to_string()).into_response()
    }
}

impl<E> From<E> for Failed
where
    E: Into<anyhow::Error>,
{
    fn from(_value: E) -> Self {
        Failed::new(PROBLEM_URI.clone(), ErrorKind::ServerError, None)
    }
}

// fn append_optional_query_pair<T>(
//     key: &str,
//     value: Option<T>,
// ) where
//     T: AsRef<str>,
// {
//     if let Some(value) = value {
//         query_pairs.append_pair(key, value.as_ref());
//     }
// }

#[axum::debug_handler]
pub async fn get(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(query): Query<AuthorizationCodeRequest>,
) -> impl IntoResponse {
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

    let (oauth_client, redirect_uri) = match (oauth_client, redirect_uri) {
        (Some(oauth_client), Some(redirect_uri)) => (oauth_client, redirect_uri),
        _ => {
            return Err(Failed::new(
                PROBLEM_URI.clone(),
                ErrorKind::InvalidRequest,
                query.state,
            ));
        }
    };

    if oauth_client.id != redirect_uri.client_id {
        return Err(Failed::new(
            PROBLEM_URI.clone(),
            ErrorKind::InvalidRequest,
            query.state,
        ));
    }

    let redirect_uri = redirect_uri.uri.parse::<Uri>().map_err(|err| {
        Failed::new(PROBLEM_URI.clone(), ErrorKind::InvalidRequest, query.state)
            .with_error_description("Parsing the provided redirect URI failed.")
    })?;

    if query.response_type != "code".to_string() {
        return Err(
            Failed::new(redirect_uri, ErrorKind::InvalidRequest, query.state)
                .with_error_description("Only 'code' is allowed as response type."),
        );
    }

    let _session = match jar.get("session") {
        Some(session) => session,
        None => return Redirect::to(get_login_url(query)),
    };

    // TODO: Verify session token

    // TODO: Generate code

    // TODO: Don't use unwrap
    let response = Success {
        uri: redirect_uri,
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
fn get_login_url(query: AuthorizationCodeRequest) -> Result<Uri> {
    // TODO: Remove - this comment is only for debugging purposes.
    // http://localhost:3001/oauth/v2.0/authorize?client_id=cdd37e5a-a554-4535-bff2-45ba130b05b4&redirect_uri=http://localhost:3000&scope=openid.

    let uri_builder = UriBuilder::new().authority("/login");

    // {
    //     let mut query_pairs = login_uri.query();
    //     query_pairs
    //         .append_pair("client_id", &query.client_id.to_string())
    //         .append_pair("redirect_uri", &query.redirect_uri)
    //         .append_pair("scope", &query.scope);
    //     append_optional_query_pair(&mut query_pairs, "state", query.state);
    // }

    login_uri
}
