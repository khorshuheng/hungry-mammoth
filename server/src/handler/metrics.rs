use crate::metrics::{self, request::HTTP_REQUESTS_DURATION};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};

pub fn setup_metrics_recorder() -> PrometheusHandle {
  let builder = PrometheusBuilder::new();
  const EXPONENTIAL_SECONDS: &[f64] = &[
    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
  ];
  let handler = builder
    .set_buckets_for_metric(
      Matcher::Full(HTTP_REQUESTS_DURATION.to_string()),
      EXPONENTIAL_SECONDS,
    )
    .unwrap()
    .install_recorder()
    .expect("failed to install Prometheus recorder");
  metrics::init_metrics();
  handler
}
