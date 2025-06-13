use axum::Json;
use lerpz_utils::axum::error::HandlerResult;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TokenGrantType {
    AC(AuthorizationCodeGrant),
    CC(ClientCredentialsGrant),
    PC(PasswordCredentialsGrant),
    IF(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationCodeGrant {
  pub grant_type: String,
  pub code: String,
  pub redirect_uri: String,
  pub client_id: String,
  pub client_secret:String 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCredentialsGrant {
  grant_type: String,
  scope: String,
  client_id: String,
  client_secret: String, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImplicitFlowGrant {
  grant_type: String,
  scope: String,
  client_id: String,
  client_secret: String, 
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordCredentialsGrant {
    pub grant_type: String,
    pub username: String,
    pub password: String,
    pub client_id: String,
    pub client_secret: String,
    pub scope: String,
}

#[axum::debug_handler]
pub async fn handler(Json(body): Json<TokenGrantType>) -> HandlerResult<()> {
    use TokenGrantType::*;

    match body {
        AC(_) => authorization_code_handler().await,
        CC(_) => client_credentials_handler().await,
        PC(_) => password_credentials_handler().await,
        IF(_) => implicit_flow_handler().await,
    }
}

pub async fn authorization_code_handler() -> HandlerResult<()> {
    Ok(())
}

pub async fn client_credentials_handler() -> HandlerResult<()> {
    Ok(())
}

pub async fn password_credentials_handler() -> HandlerResult<()> {
    Ok(())
}

pub async fn implicit_flow_handler() -> HandlerResult<()> {
    Ok(())
}
