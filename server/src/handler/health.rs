use axum::http::StatusCode;

use crate::dto::{
  health::{HealthCheckResponse, HealthCheckStatus},
  wrapper::{ApiError, ApiSuccess},
};

pub async fn health_check() -> Result<ApiSuccess<HealthCheckResponse>, ApiError> {
  Ok(ApiSuccess {
    data: HealthCheckResponse {
      result: HealthCheckStatus::Pass,
    },
    status: StatusCode::OK,
  })
}
