use axum::{routing::get, Router};

use crate::handler::health::*;

pub(crate) fn routes() -> Router {
  Router::new().route("/health", get(health_check))
}
