use axum::{
  extract::{FromRequest, Request},
  Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::request::RequestError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedRequest<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedRequest<T>
where
  T: DeserializeOwned + Validate,
  S: Send + Sync,
{
  type Rejection = RequestError;

  async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
    let Json(value) = Json::<T>::from_request(req, state).await?;
    value.validate()?;
    Ok(ValidatedRequest(value))
  }
}
