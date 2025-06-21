use lerpz_utils::axum::error::HandlerResult;

use axum::{Form, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(tag = "grant_type")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum GrantRequest {
    AuthorizationCode(AccessTokenRequest),
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum GrantResponse {
    AccessToken(AccessTokenResponse),
}

/// A request to exchange an authorization code for an access token.
/// 
/// Source: https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.3
#[derive(Deserialize, Debug)]
pub struct AccessTokenRequest {
    code: String,
    client_id: String,
    redirect_uri: String,
}

/// A response containing an access token, refresh token, and other metadata.
/// 
/// Source:https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.4
#[derive(Serialize, Debug)]
pub struct AccessTokenResponse {
    token_type: String,
    access_token: String,
    refresh_token: String,
    expires_in: u64,
}

#[axum::debug_handler]
pub async fn handler(Form(body): Form<GrantRequest>) -> HandlerResult<Json<GrantResponse>> {
    let response = match body {
        GrantRequest::AuthorizationCode(req) => authorization_code(req),
    };

    Ok(Json(GrantResponse::AccessToken(response)))
}

pub fn authorization_code(req: AccessTokenRequest) -> AccessTokenResponse {
    println!("Received authorization code: {}", req.code);
    AccessTokenResponse {
        access_token: "example_access_token".into(),
        token_type: "Bearer".into(),
        expires_in: 3600,
        refresh_token: "example_refresh_token".into(),
    }
}
