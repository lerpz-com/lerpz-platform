use axum::{
	extract::{FromRequest, Request},
	http::StatusCode,
	Form, Json,
};
use serde::{de::DeserializeOwned, Serialize};
use validator::{Validate, ValidationErrors};

use crate::axum::error::{HandlerError, HandlerResult};

/// Validator that validates the inner value.
///
/// This is using the `validator` crate to validate the inner value. Used to
/// validate the body of incoming requests.
pub struct Validated<T>(pub T);

/// Error response for validation errors.
/// 
/// This is the error response that is returned when the validation fails. It
/// is boxed to avoid large data objects since there can be alot of errors.
#[derive(Serialize, Debug, Clone)]
pub struct ValidationError {
	errors: Box<ValidationErrors>,
}

impl ValidationError {
	pub fn new(errors: ValidationErrors) -> Self {
		Self { errors: Box::new(errors) }
	}
}

impl<S, T> FromRequest<S> for Validated<Json<T>>
where
	S: Send + Sync,
	T: DeserializeOwned + Validate,
{
	type Rejection = HandlerError<ValidationError>;

	async fn from_request(r: Request, s: &S) -> Result<Self, Self::Rejection> {
		let json = Json::<T>::from_request(r, s).await.map_err(unparseable)?;
		validate(&json.0)?;
		Ok(Validated(json))
	}
}

impl<S, T> FromRequest<S> for Validated<Form<T>>
where
	S: Send + Sync,
	T: DeserializeOwned + Validate,
{
	type Rejection = HandlerError<ValidationError>;

	async fn from_request(r: Request, s: &S) -> Result<Self, Self::Rejection> {
		let form = Form::<T>::from_request(r, s).await.map_err(unparseable)?;
		validate(&form.0)?;
		Ok(Validated(form))
	}
}

#[inline]
fn validate<T: Validate>(data: T) -> HandlerResult<(), ValidationError> {
	data.validate().map_err(|err| {
		HandlerError::new(
			StatusCode::BAD_REQUEST,
			"Validation failed",
			"Couldn't validate request body.",
		)
	    .with_extension(ValidationError::new(err))
	})
}

#[inline]
fn unparseable<T: std::error::Error>(_: T) -> HandlerError<ValidationError> {
	HandlerError::new(
		StatusCode::BAD_REQUEST,
		"Unparseable request",
		"Couldn't parse request body.",
	)
}
