use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use tracing::info;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let static_dir = ServeDir::new("static");
  let app = Router::new()
    .route("/health", get(check_health))
    .fallback_service(static_dir.clone());

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

async fn check_health() -> String {
  info!("Health check.");
  "Hello from axum.".to_string()
}
