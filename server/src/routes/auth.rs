use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::handler::auth::*;
use crate::state::auth::AuthState;

pub(crate) fn routes(auth_state: AuthState) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(get_token))
    .with_state(auth_state)
}
