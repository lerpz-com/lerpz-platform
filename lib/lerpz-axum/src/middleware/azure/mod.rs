use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{decode, decode_header};
use serde::{Deserialize, Deserializer};

use crate::error::HandlerError;

pub use config::*;
pub use validation::*;

mod config;
mod validation;

/// A token representing a user in the Azure Entra system.
///
/// # Note:
/// 
/// This does not support multi-tenant applications (yet).
#[derive(Debug, Deserialize)]
pub struct AzureToken {
    pub iss: String,
    pub aud: String,
    pub exp: usize,
    pub nbf: Option<usize>,
    pub iat: Option<usize>,
    pub sub: Option<String>,
    pub tid: Option<String>,

    #[serde(default, deserialize_with = "deserialize_space_separated_scopes")]
    pub scp: Vec<String>,
    pub roles: Option<Vec<String>>,
    pub appid: Option<String>,
    pub ver: Option<String>,

    pub nonce: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
}

/// Used to deserialize the [`AzureToken::scp`].
///
/// This is a space separated string
fn deserialize_space_separated_scopes<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_string: Option<String> = Option::deserialize(deserializer)?;

    Ok(opt_string
        .map(|s| {
            s.split_whitespace()
                .map(|scope| scope.to_string())
                .collect()
        })
        .unwrap_or_default())
}

impl AzureToken {
    /// Check if the token has scope.
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scp.iter().any(|s| s == scope)
    }

    /// Check if the token has any of scopes.
    pub fn has_any_scope(&self, scopes: &[&str]) -> bool {
        scopes.iter().any(|scope| self.has_scope(scope))
    }

    /// Check if the token has scope.
    ///
    /// This will return [`HandlerError::unauthorized()`] if scope is not found.
    pub fn has_scope_unauthorized(&self, scope: &str) -> Result<(), HandlerError> {
        self.has_scope(scope)
            .then_some(())
            .ok_or(HandlerError::unauthorized())
    }

    /// Check if the token has any of scopes.
    ///
    /// This will return [`HandlerError::unauthorized()`] if all scopes are not found.
    pub fn has_any_scope_unauthorized(&self, scopes: &[&str]) -> Result<(), HandlerError> {
        self.has_any_scope(scopes)
            .then_some(())
            .ok_or(HandlerError::unauthorized())
    }

    /// Check if the token has role.
    pub fn has_role(&self, role: &str) -> bool {
        self.roles
            .as_ref()
            .map(|roles| roles.contains(&role.to_string()))
            .unwrap_or(false)
    }

    /// Check if the token has any of roles.
    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        roles.iter().any(|role| self.has_role(role))
    }

    /// Check if the token has role.
    ///
    /// This will return [`HandlerError::unauthorized()`] if role is not found.
    pub fn has_role_unauthorized(&self, role: &str) -> Result<(), HandlerError> {
        self.has_role(role)
            .then_some(())
            .ok_or(HandlerError::unauthorized())
    }

    /// Check if the token has any of roles.
    ///
    /// This will return [`HandlerError::unauthorized()`] if all roles are not found.
    pub fn has_any_role_unauthorized(&self, roles: &[&str]) -> Result<(), HandlerError> {
        self.has_any_role(roles)
            .then_some(())
            .ok_or(HandlerError::unauthorized())
    }
}

impl<S> FromRequestParts<S> for AzureToken
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
        let token_data = decode::<AzureToken>(token, &decoding_key, &validation)
            .map_err(|_| HandlerError::unauthorized())?;

        if !custom_validation(&config, &token_data.claims) {
            return Err(HandlerError::unauthorized());
        }

        Ok(token_data.claims)
    }
}
