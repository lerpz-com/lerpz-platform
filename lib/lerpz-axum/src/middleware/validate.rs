use std::{borrow::Cow, collections::HashMap};

use axum::{
    Form, Json,
    extract::{FromRequest, Request},
    http::StatusCode,
};
use serde::{Serialize, de::DeserializeOwned};
use validator::{Validate, ValidationErrors, ValidationErrorsKind};

use crate::error::{HandlerError, HandlerResult};

/// Validator that validates the inner value.
///
/// This is using the [`validator`] crate.
pub struct Validated<T>(pub T);

/// Error response for validation errors.
/// 
/// This is used to return validation errors in a structured format. The format
/// is a map of field names to a list of errors for that field.
#[derive(Serialize, Debug, Clone)]
pub struct ValidationErrorResponse {
    pub validation_errors: HashMap<Cow<'static, str>, FieldErrors>,
}

/// Errors in the individual fields.
#[derive(Serialize, Debug, Clone)]
pub struct FieldErrors(pub Vec<Cow<'static, str>>);


impl From<ValidationErrors> for ValidationErrorResponse {
    fn from(ValidationErrors(errors): ValidationErrors) -> Self {
        let mut error_map = HashMap::new();

        for (field, kind) in errors {
            error_map.insert(field.into(), kind.into());
        }

        Self { validation_errors: error_map }
    }
}

impl From<ValidationErrorsKind> for FieldErrors {
    fn from(err: ValidationErrorsKind) -> Self {
		dbg!(&err);
        match err {
            ValidationErrorsKind::Field(errors) => FieldErrors(
                errors
                    .into_iter()
                    .map(|e| e.message.unwrap_or_else(|| "Unkown validation error".into()))
                    .collect(),
            ),
            _ => todo!(),
        }
    }
}

impl<S, T> FromRequest<S> for Validated<Json<T>>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = HandlerError<ValidationErrorResponse>;

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
    type Rejection = HandlerError<ValidationErrorResponse>;

    async fn from_request(r: Request, s: &S) -> Result<Self, Self::Rejection> {
        let form = Form::<T>::from_request(r, s).await.map_err(unparseable)?;
        validate(&form.0)?;
        Ok(Validated(form))
    }
}

/// Validates the given data.
#[inline]
fn validate<T: Validate>(data: T) -> HandlerResult<(), ValidationErrorResponse> {
    data.validate().map_err(|err| {
        HandlerError::new(
            StatusCode::BAD_REQUEST,
            "Validation failed",
            "Couldn't validate request body.",
        )
        .with_extension(ValidationErrorResponse::from(err))
    })
}

/// Returns a `HandlerError` for unparseable requests.
#[inline]
fn unparseable<T: std::error::Error>(_: T) -> HandlerError<ValidationErrorResponse> {
    HandlerError::new(
        StatusCode::BAD_REQUEST,
        "Unparseable request",
        "Couldn't parse request body.",
    )
}
