use axum::Json;

use crate::{
  dto::health::{HealthCheckResponse, HealthCheckStatus},
  response::api_response::ApiSuccessResponse,
};

pub async fn check() -> Json<ApiSuccessResponse<HealthCheckResponse>> {
  Json(ApiSuccessResponse::send(HealthCheckResponse {
    status: HealthCheckStatus::Pass,
  }))
}
