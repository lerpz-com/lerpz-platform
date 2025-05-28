use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represent all claims for a token.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    /// Subject of the token.
    pub sub: Uuid,
    /// Which time the token will expire.
    pub exp: i64,
    /// Which time the token should be valid for.
    pub nbf: i64,
    /// What time the token was issued.
    pub iat: i64,
    /// Who issued the token.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub iss: String,
    /// What audience the token is for.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub aud: String,
}
