use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{decode, decode_header};
use serde::Deserialize;

use crate::error::HandlerError;

pub use config::*;
pub use validation::*;

mod config;
mod validation;

#[derive(Debug, Deserialize)]
pub struct AzureUser {
    pub iss: String,
    pub aud: String,
    pub exp: usize,
    pub nbf: Option<usize>,
    pub iat: Option<usize>,
    pub sub: Option<String>,
    pub tid: Option<String>,

    pub scp: Option<String>,
    pub roles: Option<Vec<String>>,
    pub appid: Option<String>,
    pub ver: Option<String>,

    pub nonce: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
}

impl<S> FromRequestParts<S> for AzureUser
where
    AzureConfig: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = HandlerError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or_else(|| HandlerError::unauthorized())?;

        let header = decode_header(token).map_err(|_| HandlerError::unauthorized())?;
        let kid = header.kid.ok_or(HandlerError::unauthorized())?;

        let config = AzureConfig::from_ref(state);
        let decoding_key = config
            .get_jwk(kid)
            .await?
            .ok_or(HandlerError::unauthorized())?;

        let validation = get_token_validation(&config);
        let token_data = decode::<AzureUser>(token, &decoding_key, &validation)
            .map_err(|_| HandlerError::unauthorized())?;

        Ok(token_data.claims)
    }
}
