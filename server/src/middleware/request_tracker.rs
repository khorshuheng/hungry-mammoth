use std::time::Instant;

use axum::{
  extract::{MatchedPath, Request},
  middleware::Next,
  response::IntoResponse,
};

use crate::metrics;

pub async fn track_requests(req: Request, next: Next) -> impl IntoResponse {
  let start = Instant::now();
  let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
    matched_path.as_str().to_owned()
  } else {
    req.uri().path().to_owned()
  };
  let method = req.method().clone();
  let resp = next.run(req).await;
  let latency = start.elapsed();

  metrics::request::increase_http_requests_total(&method, &path, &resp.status());
  metrics::request::record_http_requests_duration(&method, &path, &resp.status(), &latency);

  resp
}
