use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use sqlx::{PgPool, Postgres, pool::PoolConnection};

use crate::axum::error::HandlerError;

/// A wrapper around a [`PoolConnection`] to make it extractable from request state.
///
/// This is useful when you want to use a database connection in your handlers.
/// We force the connection type to be a Postgres connection here.
pub struct Database(pub PoolConnection<Postgres>);

impl<S> FromRequestParts<S> for Database
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = HandlerError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);
        let conn = pool.acquire().await.map_err(HandlerError::from)?;
        Ok(Self(conn))
    }
}
