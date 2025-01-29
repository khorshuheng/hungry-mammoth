use std::future::ready;

use axum::{routing::get, Router};

use crate::handler::metrics::setup_metrics_recorder;

pub fn routes() -> Router {
  let recorder_handle = setup_metrics_recorder();
  Router::new().route("/metrics", get(move || ready(recorder_handle.render())))
}
