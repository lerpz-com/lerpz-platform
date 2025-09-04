use jsonwebtoken::{Algorithm, Validation};

/// Default JWT validation claims.
///
/// The algorithm will always be RS256.
pub fn get_token_validation(config: &super::AzureConfig) -> Validation {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_required_spec_claims(&["aud", "iss", "exp"]);
    validation.set_audience(&[&config.client_id]);
    validation.set_issuer(&[&config.get_issuer_url()]);
    validation.validate_exp = true;
    validation.validate_nbf = true;
    validation.leeway = 60;
    validation
}

/// Custom validation other than the default JWT claims.
pub fn custom_validation(config: &super::AzureConfig, claims: &super::AzureToken) -> bool {
    if claims.ver.as_deref() != Some("2.0") {
        return false;
    }

    if let Some(tid) = &claims.tid {
        if tid != &config.tenant_id {
            return false;
        }
    } else {
        return false;
    }

    if let Some(sub) = &claims.sub {
        if sub.is_empty() {
            return false;
        }
    }

    true
}
