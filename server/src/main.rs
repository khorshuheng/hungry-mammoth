use tokio::signal;
use tracing::info;

mod dto;
mod handler;
mod metrics;
mod middleware;
mod response;
mod routes;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();
  let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
  tokio::join!(
    start_app_server(shutdown_tx),
    start_metrics_server(shutdown_rx)
  );
}

async fn start_app_server(shutdown_tx: tokio::sync::oneshot::Sender<()>) {
  let app_routes = routes::root::routes();
  let app_server_listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
  info!("App server started on port 8000");
  axum::serve(app_server_listener, app_routes)
    .with_graceful_shutdown(app_server_shutdown_handler(shutdown_tx))
    .await
    .unwrap();
}

async fn start_metrics_server(shutdown_rx: tokio::sync::oneshot::Receiver<()>) {
  let metric_routes = routes::metrics::routes();
  let metrics_server_listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
  info!("Metrics server started on port 8080");
  axum::serve(metrics_server_listener, metric_routes)
    .with_graceful_shutdown(metric_server_shutdown_handler(shutdown_rx))
    .await
    .unwrap();
}

async fn app_server_shutdown_handler(shutdown_tx: tokio::sync::oneshot::Sender<()>) {
  let ctrl_c = async {
    signal::ctrl_c()
      .await
      .expect("failed to install Ctrl+C handler");
  };

  #[cfg(unix)]
  let terminate = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .expect("failed to install signal handler")
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
      _ = ctrl_c => {
        info!("Received Ctrl+C signal, shutting down");
        shutdown_tx.send(()).unwrap();
      },
      _ = terminate => {
        info!("Received terminate signal, shutting down");
        shutdown_tx.send(()).unwrap();
      },
  }
  info!("App server shut down");
}

async fn metric_server_shutdown_handler(shutdown_rx: tokio::sync::oneshot::Receiver<()>) {
  shutdown_rx.await.unwrap();
  info!("Metrics server shut down");
}
