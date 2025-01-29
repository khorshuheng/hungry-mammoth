use serde::Serialize;

#[derive(Serialize)]
pub enum HealthCheckStatus {
  Pass,
  #[allow(dead_code)]
  Fail,
}

#[derive(Serialize)]
pub struct HealthCheckResponse {
  pub status: HealthCheckStatus,
}
