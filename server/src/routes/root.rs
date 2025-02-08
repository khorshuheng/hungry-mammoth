use axum::{routing::IntoMakeService, Router};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{middleware::request_tracker::track_requests, state::AppState};

use super::{health, user};

pub fn routes(app_state: AppState) -> IntoMakeService<Router> {
  let AppState { user_state } = app_state;

  let health_router = health::routes();
  let user_router = user::routes(user_state);
  let merged_router = health_router
    .merge(user_router)
    .route_layer(axum::middleware::from_fn(track_requests));
  let static_dir = ServeDir::new("static");
  let app_router = Router::new()
    .nest("/api", merged_router)
    .fallback_service(static_dir)
    .layer(TraceLayer::new_for_http());

  app_router.into_make_service()
}
