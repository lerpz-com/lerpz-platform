//! Functions related to token generation and verification.

/// Claims related to JWT tokens.
pub mod claims;
/// Errors that can occur when working with JWT tokens.
pub mod error;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};

pub use claims::Claims;
pub use error::{Error, Result};

pub fn encode_jwt(claims: impl Into<Claims>, key: &EncodingKey) -> Result<String> {
    let header = Header::default();
    let claims = claims.into();
    let token = encode(&header, &claims, key).map_err(Error::TokenError)?;
    Ok(token)
}

pub fn decode_jwt(token: &str, key: &DecodingKey) -> Result<TokenData<Claims>> {
    let validation = Validation::default();
    let claims = decode::<Claims>(token, key, &validation).map_err(Error::TokenError)?;
    Ok(claims)
}
