use axum::{
  extract::rejection::JsonRejection,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::dto::wrapper::ApiError;

#[derive(Debug, Error)]
pub enum RequestError {
  #[error(transparent)]
  ValidationError(#[from] validator::ValidationErrors),
  #[error(transparent)]
  JsonRejection(#[from] JsonRejection),
}

impl IntoResponse for RequestError {
  fn into_response(self) -> Response {
    match self {
      RequestError::ValidationError(_) => ApiError::send(
        StatusCode::BAD_REQUEST,
        Some(self.to_string().replace('\n', ", ")),
      ),
      RequestError::JsonRejection(_) => {
        ApiError::send(StatusCode::BAD_REQUEST, Some(self.to_string()))
      },
    }
  }
}
