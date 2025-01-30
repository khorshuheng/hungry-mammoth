use std::future::ready;

use axum::{routing::get, Router};

use crate::{config::MetricsConfig, handler::metrics::setup_metrics_recorder};

pub fn routes(metrics_config: &MetricsConfig) -> Router {
  let recorder_handle = setup_metrics_recorder(metrics_config);
  Router::new().route("/metrics", get(move || ready(recorder_handle.render())))
}
