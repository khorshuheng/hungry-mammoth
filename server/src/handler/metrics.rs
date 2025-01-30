use crate::{
  config::MetricsConfig,
  metrics::{self, request::HTTP_REQUESTS_DURATION},
};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};

pub fn setup_metrics_recorder(metrics_config: &MetricsConfig) -> PrometheusHandle {
  let builder = PrometheusBuilder::new();
  let handler = builder
    .set_buckets_for_metric(
      Matcher::Full(HTTP_REQUESTS_DURATION.to_string()),
      metrics_config.http_requests_latency_buckets.as_slice(),
    )
    .unwrap()
    .install_recorder()
    .expect("failed to install Prometheus recorder");
  metrics::init_metrics();
  handler
}
