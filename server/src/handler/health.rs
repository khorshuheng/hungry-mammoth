use axum::http::StatusCode;

use crate::dto::{
  health::{HealthCheckResponse, HealthCheckStatus},
  wrapper::{ApiError, ApiSuccess},
};

#[utoipa::path(
  tag = "Health",
  get,
  path = "/health",
  responses(
    (status = 200, description = "Heath check passed", body = HealthCheckResponse)
  )
)]
pub async fn health_check() -> Result<ApiSuccess<HealthCheckResponse>, ApiError> {
  Ok(ApiSuccess {
    data: HealthCheckResponse {
      result: HealthCheckStatus::Pass,
    },
    status: StatusCode::OK,
  })
}
