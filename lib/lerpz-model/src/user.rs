use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub primary_email: String,
    pub password_hash: String,
    pub password_salt: String,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub organization_id: Option<Uuid>,
}
