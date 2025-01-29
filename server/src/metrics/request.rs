use std::time::Duration;

use axum::http::{self, StatusCode};
use metrics::{counter, describe_counter, histogram};

pub static HTTP_REQUESTS_TOTAL: &str = "http_requests_total";
pub static HTTP_REQUESTS_DURATION: &str = "http_requests_duration";

pub(crate) fn init_metrics() {
  describe_counter!(HTTP_REQUESTS_TOTAL, "Total number of http requests.");
  describe_counter!(HTTP_REQUESTS_DURATION, "Http requests duration in seconds.");
}

pub fn increase_http_requests_total(method: &http::Method, path: &str, status_code: &StatusCode) {
  let labels = [
    ("method", method.to_string()),
    ("path", path.to_string()),
    ("status", status_code.as_u16().to_string()),
  ];
  counter!(HTTP_REQUESTS_TOTAL, &labels).increment(1);
}

pub fn record_http_requests_duration(
  method: &http::Method,
  path: &str,
  status_code: &StatusCode,
  latency: &Duration,
) {
  let labels = [
    ("method", method.to_string()),
    ("path", path.to_string()),
    ("status", status_code.as_u16().to_string()),
  ];
  histogram!(HTTP_REQUESTS_DURATION, &labels).record(latency.as_secs_f64());
}
