use axum::http::StatusCode;
use thiserror::Error;

use crate::dto::wrapper::ApiError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ServiceError {
  #[error("internal error: {0}")]
  InternalError(String),
  #[allow(dead_code)]
  #[error("unprocessable entity: {0}")]
  UnprocessableEntity(String),
  #[error("entity not found")]
  EntityNotFound,
}

impl From<ServiceError> for ApiError {
  fn from(error: ServiceError) -> Self {
    match error {
      ServiceError::InternalError(_) => ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: error.to_string(),
      },
      ServiceError::UnprocessableEntity(_) => ApiError {
        status: StatusCode::UNPROCESSABLE_ENTITY,
        message: error.to_string(),
      },
      ServiceError::EntityNotFound => ApiError {
        status: StatusCode::NOT_FOUND,
        message: error.to_string(),
      },
    }
  }
}
