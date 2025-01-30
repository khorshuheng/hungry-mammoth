use crate::dto::{
  health::{HealthCheckResponse, HealthCheckStatus},
  wrapper::{ApiResult, ApiSuccess},
};

pub async fn health_check() -> ApiResult<HealthCheckResponse> {
  Ok(ApiSuccess::send(HealthCheckResponse {
    status: HealthCheckStatus::Pass,
  }))
}
