//! The token endpoint for OAuth 2.0
//! 
//! Only the Authorization Code, Client Credentials, and Refresh Token flows are
//! implemented. The Password Grant flow is not implemented as it is deprecated.

use lerpz_axum::error::{HandlerError, HandlerResult};

use axum::{Form, Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(tag = "grant_type")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum GrantType {
    AuthorizationCode(AuthorizationCodeRequest),
    ClientCredentials(ClientCredentialsRequest),
    RefreshToken(RefreshTokenRequest),
}

/// A request to exchange an authorization code for an access token.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.3
#[derive(Deserialize, Debug)]
pub struct AuthorizationCodeRequest {
    code: String,
    redirect_uri: String,
    client_id: String,
}

/// A request to exchange client credentials for an access token.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.4.2
#[derive(Deserialize, Debug)]
pub struct ClientCredentialsRequest {
    client_id: String,
    client_secret: String,
    scope: Option<String>,
}

/// A request to exchange username and password for an access token.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-6
#[derive(Deserialize, Debug)]
pub struct RefreshTokenRequest {
    refresh_token: String,
    scope: String,
}

/// A response containing an access token, refresh token, and other metadata.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-5.1
#[derive(Serialize, Debug)]
pub struct AccessTokenResponse {
    /// The access token issued by the authorization server.
    access_token: String,
    /// The type of the token issued, typically "Bearer".
    ///
    /// Sources:
    /// - https://datatracker.ietf.org/doc/html/rfc6749#section-7.1
    token_type: String,
    /// The lifetime in seconds of the access token.
    ///
    /// This might not be present if the lifetime is defined within the token
    /// itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_in: Option<u64>,
    /// A token that can be used to obtain a new access token without re-authenticating.
    ///
    /// This is not always present, depending on the grant type.
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,
    /// Scope of the requested access token.
    ///
    /// Might not be present if the scope is the same as the one requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
}

#[axum::debug_handler]
pub async fn handler(Form(body): Form<GrantType>) -> HandlerResult<Json<AccessTokenResponse>> {
    let access_token = match body {
        GrantType::AuthorizationCode(req) => authorization_code(req),
        GrantType::ClientCredentials(req) => client_credentials(req),
        GrantType::RefreshToken(req) => refresh_token(req),
    }?;

    Ok(Json(access_token))
}

fn authorization_code(_req: AuthorizationCodeRequest) -> HandlerResult<AccessTokenResponse> {
    Err(HandlerError::new(
        StatusCode::NOT_IMPLEMENTED,
        "Flow not implemented",
        "Authorization code flow is not implemented yet.",
    ))
}

fn client_credentials(_req: ClientCredentialsRequest) -> HandlerResult<AccessTokenResponse> {
    Err(HandlerError::new(
        StatusCode::NOT_IMPLEMENTED,
        "Flow not implemented",
        "Client credentials flow is not implemented yet.",
    ))
}

fn refresh_token(_req: RefreshTokenRequest) -> Result<AccessTokenResponse, HandlerError> {
    Err(HandlerError::new(
        StatusCode::NOT_IMPLEMENTED,
        "Flow not implemented",
        "Refresh token flow is not implemented yet.",
    ))
}
