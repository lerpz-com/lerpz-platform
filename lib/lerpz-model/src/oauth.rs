use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Clone)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct OauthClients {
    pub id: Uuid,
    pub secret: String,
    pub name: String,
    pub description: Option<String>,
    pub organization_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct UserClientAuthorizations {
    pub id: Uuid,
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub scope: String,
    pub authorized_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct RedirectUris {
    pub id: Uuid,
    pub client_id: Uuid,
    pub uri: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct Scopes {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_scope_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct ClientScopes {
    pub client_id: Uuid,
    pub scope_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct RefreshTokens {
    pub id: Uuid,
    pub token: String,
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub revoked_at: Option<NaiveDateTime>,
}

pub struct AccessTokens {
    pub id: Uuid,
    pub jti: String,
    pub user_id: Option<Uuid>,
    pub client_id: Uuid,
    pub scope: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub revoked_at: Option<NaiveDateTime>,
}
