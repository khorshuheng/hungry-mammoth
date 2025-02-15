use axum::{routing::IntoMakeService, Router};
use tower_http::{services::ServeDir, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable};

use crate::{middleware::request_tracker::track_requests, state::AppState};

use super::{auth, health, user};

#[derive(OpenApi)]
#[openapi(
    info(
      license(identifier = "MIT"),
    ),
    servers(
      (url = "http://localhost:8000"),
    ),
)]
struct ApiDoc;

pub fn routes(app_state: AppState) -> IntoMakeService<Router> {
  let AppState { user_state } = app_state;

  let health_router = health::routes();
  let user_router = user::routes(user_state);
  let auth_router = auth::routes();
  let merged_router = health_router
    .merge(user_router)
    .merge(auth_router)
    .route_layer(axum::middleware::from_fn(track_requests));
  let static_dir = ServeDir::new("static");
  let (app_router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
    .nest("/api", merged_router)
    .fallback_service(static_dir)
    .layer(TraceLayer::new_for_http())
    .split_for_parts();
  let api_clone = api.clone();
  let api_json_router = Router::new().route(
    "/openapi.json",
    axum::routing::get(move || async { axum::response::Json(api_clone) }),
  );
  let app_router = app_router
    .merge(Redoc::with_url("/redoc", api))
    .merge(api_json_router);
  app_router.into_make_service()
}
