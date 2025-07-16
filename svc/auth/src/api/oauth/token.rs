use lerpz_utils::axum::error::{HandlerError, HandlerResult};

use axum::{Form, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(tag = "grant_type")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum GrantRequest {
    AuthorizationCode(AuthorizationCodeRequest),
    PasswordCredentials(PasswordCredentialsRequest),
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

/// A request to exchange username and password for an access token.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.3.2
#[derive(Deserialize, Debug)]
pub struct PasswordCredentialsRequest {
    password: String,
    username: String,
    scope: String,
}

/// A request to exchange client credentials for an access token.
///
/// Sources:
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-4.4.2
#[derive(Deserialize, Debug)]
pub struct ClientCredentialsRequest {
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
    /// Might not be present if the scope is the same as the one requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
}

#[axum::debug_handler]
pub async fn handler(Form(body): Form<GrantRequest>) -> HandlerResult<Json<AccessTokenResponse>> {
    let access_token = match body {
        GrantRequest::AuthorizationCode(req) => authorization_code(req),
        GrantRequest::PasswordCredentials(req) => password_credentials(req),
        GrantRequest::ClientCredentials(req) => client_credentials(req),
        GrantRequest::RefreshToken(req) => refresh_token(req),
    }?;

    Ok(Json(access_token))
}

 fn authorization_code(req: AuthorizationCodeRequest) -> Result<AccessTokenResponse, HandlerError> {
    println!("Received authorization code: {}", req.code);
    Ok(AccessTokenResponse {
        access_token: "example_access_token".into(),
        token_type: "Bearer".into(),
        expires_in: None,
        refresh_token: Some("example_refresh_token".into()),
        scope: None,  
    })
}

fn password_credentials(req: PasswordCredentialsRequest) -> Result<AccessTokenResponse, HandlerError> {
    println!("Received password credentials: {}:{}", req.username, req.password);
    Ok(AccessTokenResponse {
        access_token: "example_access_token".into(),
        token_type: "Bearer".into(),
        expires_in: None,
        refresh_token: Some("example_refresh_token".into()),
        scope: None,  
    })
}

fn client_credentials(_req: ClientCredentialsRequest) -> Result<AccessTokenResponse, HandlerError> {
    println!("Received client credentials");
    Ok(AccessTokenResponse {
        access_token: "example_access_token".into(),
        token_type: "Bearer".into(),
        expires_in: None,
        refresh_token: Some("example_refresh_token".into()),
        scope: None,  
    })
}


fn refresh_token(req: RefreshTokenRequest) -> Result<AccessTokenResponse, HandlerError> {
    println!("Received refresh token: {}", req.refresh_token);
    Ok(AccessTokenResponse {
        access_token: "example_access_token".into(),
        token_type: "Bearer".into(),
        expires_in: None,
        refresh_token: Some("example_refresh_token".into()),
        scope: None,  
    })
}
