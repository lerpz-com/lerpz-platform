use serde::{Deserialize, Serialize};

/// Represent all claims for a token.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    /// What audience the token is for.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub aud: String,
    /// Who issued the token.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub iss: String,
    /// Subject of the token.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub sub: String,
    /// Which time the token will expire.
    pub exp: i64,
    /// When the token will be valid.
    pub nbf: i64,
    /// When the token was issued.
    pub iat: i64,
}
