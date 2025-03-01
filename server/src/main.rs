use hungry_mammoth::{config::AppConfig, routes, state::AppState};
use tokio::signal;
use tracing::{debug, info};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  tracing_subscriber::fmt::init();
  let app_config = AppConfig::new().expect("error parsing configuration");
  debug!("App config: {:?}", app_config);
  let app_state = AppState::new(&app_config).await;

  let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
  tokio::join!(
    start_app_server(shutdown_tx, &app_config, app_state),
    start_metrics_server(shutdown_rx, &app_config)
  );
}

async fn start_app_server(
  shutdown_tx: tokio::sync::oneshot::Sender<()>,
  app_config: &AppConfig,
  app_state: AppState,
) {
  let app_routes = routes::root::routes(app_state);
  let app_listener_address = app_config.server.app_listener_address();
  let app_server_listener = tokio::net::TcpListener::bind(&app_listener_address)
    .await
    .expect("error binding to app server listener");
  info!("App server listening on {}", app_listener_address);
  axum::serve(app_server_listener, app_routes)
    .with_graceful_shutdown(app_server_shutdown_handler(shutdown_tx))
    .await
    .unwrap();
}

async fn start_metrics_server(
  shutdown_rx: tokio::sync::oneshot::Receiver<()>,
  app_config: &AppConfig,
) {
  let metric_routes = routes::metrics::routes(&app_config.metrics);
  let metrics_listener_address = app_config.server.metrics_listener_address();
  let metrics_server_listener = tokio::net::TcpListener::bind(&metrics_listener_address)
    .await
    .expect("error binding to metrics server listener");
  info!("Metrics server listening on {}", metrics_listener_address);
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
