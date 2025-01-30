use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

pub type ApiResult<T> = Result<Json<ApiSuccess<T>>, ApiError>;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ApiSuccess<T: Serialize> {
  data: T,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ApiError {
  message: Option<String>,
  #[serde(rename = "code")]
  status: u16,
}

impl<T: Serialize> ApiSuccess<T>
where
  T: Serialize,
{
  pub(crate) fn send(data: T) -> Json<ApiSuccess<T>> {
    Json(ApiSuccess { data })
  }
}

impl ApiError {
  pub(crate) fn send(status: StatusCode, message: Option<String>) -> Response {
    ApiError {
      message,
      status: status.as_u16(),
    }
    .into_response()
  }
}

impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
    (
      StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
      Json(self),
    )
      .into_response()
  }
}
