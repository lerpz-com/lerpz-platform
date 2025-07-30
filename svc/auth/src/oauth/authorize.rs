//! Endpoint for OAuth 2.0 authorization.
//!
//! This only implements the Authorization Code + PKCE flow as per RFC 6749 and
//! RFC 7636. The implicit grant is deprecated and not implemented.

use axum::{
    extract::{Query, State},
    response::Redirect,
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use url::Url;
use uuid::Uuid;

use crate::state::AppState;

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
    scope: String,
    state: Option<String>,
}

// /// A response to an authorization code request.
// ///
// /// Sources:
// /// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
// #[derive(Serialize, Debug)]
// pub enum AuthorizationCodeResponse {
//     Success {
//         code: String,
//         state: Option<String>,
//     },
//     Failed {
//         error: AuthorizationErrorKind,
//         error_description: Option<String>,
//         error_uri: Option<String>,
//         state: Option<String>,
//     },
// }

// /// Error kinds that the authorization endpoint might return.
// ///
// /// Sources:
// /// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
// #[derive(Serialize, Debug)]
// #[serde(rename_all = "snake_case")]
// pub enum AuthorizationErrorKind {
//     InvalidRequest,
//     UnauthorizedClient,
//     AccessDenied,
//     UnsupportedResponseType,
//     InvalidScope,
//     ServerError,
//     TemporarilyUnavailable,
// }

#[axum::debug_handler]
pub async fn get(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(query): Query<AuthorizationCodeRequest>,
) -> Result<Redirect, Redirect> {
    let mut database = state
        .database
        .acquire()
        .await
        .map_err(|_| Redirect::to("/problem?error=server_error"))?;

    let oauth_client = sqlx::query_as!(
        lerpz_model::OAuthClient,
        "SELECT * FROM oauth_clients WHERE id = $1",
        &query.client_id
    )
    .fetch_one(&mut *database)
    .await
    .map_err(|_| Redirect::to("/problem?error=invalid_request"))?;

    let redirect_uri = sqlx::query_as!(
        lerpz_model::RedirectUri,
        "SELECT * FROM redirect_uris WHERE uri = $1",
        &query.redirect_uri
    )
    .fetch_one(&mut *database)
    .await
    .map_err(|_| Redirect::to("/problem?error=invalid_request"))?;

    if oauth_client.id != redirect_uri.client_id {
        return Err(Redirect::to("/problem?error=access_denied"));
    }

    let _session = match jar.get("session") {
        Some(session) => session,
        None => return Ok(get_login_url(query)),
    };

    // TODO: Verify session token

    // TODO: Generate code

    // TODO: Don't use unwrap
    let mut redirect_uri = Url::parse(&redirect_uri.uri).unwrap();
    redirect_uri
        .query_pairs_mut()
        .append_pair("code", "secret_code");

    if let Some(ref state) = query.state {
        redirect_uri.query_pairs_mut().append_pair("state", state);
    }

    Ok(Redirect::to(redirect_uri.as_str()))
}

/// Generate a login URL for the given authorization code request.
fn get_login_url(query: AuthorizationCodeRequest) -> Redirect {
    // TODO: Remove - this comment is only for debugging purposes.
    // http://localhost:3001/oauth/v2.0/authorize?client_id=cdd37e5a-a554-4535-bff2-45ba130b05b4&redirect_uri=http://localhost:3000&scope=openid.

    // TODO: Make sure the login URL is not hardcoded.
    let mut login_uri = Url::parse("http://localhost:3001/login").unwrap();
    login_uri
        .query_pairs_mut()
        .append_pair("client_id", &query.client_id.to_string())
        .append_pair("redirect_uri", &query.redirect_uri)
        .append_pair("scope", &query.scope);

    if let Some(state) = query.state {
        login_uri.query_pairs_mut().append_pair("state", &state);
    }

    Redirect::to(login_uri.as_str())
}
