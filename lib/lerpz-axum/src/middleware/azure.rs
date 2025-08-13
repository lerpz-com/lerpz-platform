use axum::extract::{FromRequest, Request};

use crate::error::HandlerError;

pub struct Azure;

impl<S> FromRequest<S> for Azure
where
    S: Send + Sync,
{
    type Rejection = HandlerError<()>;

    async fn from_request(r: Request, s: &S) -> Result<Self, Self::Rejection> {
        todo!()
    }
}
