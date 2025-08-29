use jsonwebtoken::{Algorithm, Validation};

pub fn get_token_validation(config: &super::AzureConfig) -> Validation {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_required_spec_claims(&[
        "aud", "iss", "iat", "nbf", "exp", "aio", "azp", "scp", "sub", "tid", "uti", "ver",
    ]);
    validation.set_audience(&[&config.client_id]);
    validation.set_issuer(&[&config.get_issuer_url()]);
    validation.validate_exp = true;
    validation.validate_nbf = true;
    validation
}
