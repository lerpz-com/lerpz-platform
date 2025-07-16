//! Functions related to password hashing and verification.
//! 
//! The schemes are:
//! - `01`: The default scheme, which is using Argon2 (DEFAULT).

/// Errors that can occur when working with passwords.
mod error;
/// Parts needed for hashing and validating passwords.
mod parts;
/// Schemas for hashing and validating passwords.
mod scheme;

use std::{str::FromStr, sync::Arc};

pub use error::{Error, Result};
pub use parts::{HashParts, PwdParts};
pub use scheme::{Scheme, get_scheme};

/// Default scheme used for hashing passwords.
pub static DEFAULT_SCHEME: &str = "01";

/// Hash a password using the latest scheme.
pub async fn hash_pwd(pwd: impl Into<String>, salt: impl Into<String>) -> Result<String> {
    hash_pwd_parts(PwdParts::new(pwd.into(), salt.into())).await
}

/// Hash a password using custom [`PwdParts`].
///
/// This function can create a password using an old scheme. For this reason
/// this is not recommended to use.
///
/// You can use this function together with the [`PwdParts::new`] method to
/// create a password using the latest scheme.
pub async fn hash_pwd_parts(pwd_parts: PwdParts) -> Result<String> {
    tokio::task::spawn_blocking(move || {
        get_scheme(&pwd_parts.scheme)?
            .hash(&pwd_parts.pwd, &pwd_parts.salt)
            .map(|hash| format!("#{}#{}", pwd_parts.scheme, hash))
            .map_err(Error::SchemeError)
    })
    .await
    .map_err(|_| Error::FailSpawnBlockForHash)
    .and_then(|res| res)
}

/// Validate a hash against a password and a salt.
///
/// The hash needs to be parseable into [`HashParts`]. See
/// [`HashParts::from_str`] to see how the format works.
pub async fn validate_pwd(
    pwd_hash: &str,
    pwd_salt: &str,
    pwd_ref: &str,
) -> Result<bool> {
    let pwd_hash = HashParts::from_str(pwd_hash)?;
    let pwd_ref = pwd_ref.into();
    let pwd_salt = pwd_salt.into();
    validate_pwd_parts(pwd_hash, pwd_ref, pwd_salt).await
}

/// Validate a password using [`HashParts`] and a password reference.
///
/// This function validates a password hash against a password and optional
/// salt. This uses the [`HashParts`] to decide which scheme to use for
/// validating. You can use the [`HashParts::from_str`] to create the
/// [`HashParts`] needed for validating the password scheme. If you do not use
/// the correct scheme for the password, this function will error.
///
/// # Note
///
/// Make sure you use [`HashParts::from_str`] to get the scheme or be certain
/// that the scheme given is the same as what was used to create the password
/// hash.
pub async fn validate_pwd_parts(
    hash_parts: impl Into<HashParts>,
    pwd_ref: Arc<str>, 
    pwd_salt: Arc<str>,
) -> Result<bool> {
    let hash_parts = hash_parts.into();
    tokio::task::spawn_blocking(move || {
        get_scheme(&hash_parts.scheme)?
            .validate(&hash_parts.hash, &pwd_ref, &pwd_salt)
            .map_err(Error::SchemeError)
    })
    .await
    .map_err(|_| Error::FailSpawnBlockForValidate)
    .and_then(|res| res)
}

#[cfg(test)]
mod tests {
    use super::{hash_pwd, validate_pwd};

    #[tokio::test]
    async fn test_password_hashing_and_validate() {
        dotenvy::from_filename(".env.test").unwrap();

        let salt = uuid::Uuid::new_v4().to_string();
        let hash = hash_pwd("password".to_string(), salt.clone())
            .await
            .unwrap();

        assert!(!validate_pwd(&hash, &salt, "drowssap").await.unwrap());
        assert!(validate_pwd(&hash, &salt, "password").await.unwrap());
    }
}
