use axum::{routing::get, Router};

use crate::handler;

pub(crate) fn routes() -> Router {
  Router::new().route("/health", get(handler::health::check))
}
