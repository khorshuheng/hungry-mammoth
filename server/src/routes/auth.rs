use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::handler::auth::*;

pub(crate) fn routes() -> OpenApiRouter {
  OpenApiRouter::new().routes(routes!(get_token))
}
