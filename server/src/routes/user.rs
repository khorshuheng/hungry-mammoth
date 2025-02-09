use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::{handler::user::*, state::user::UserState};

pub(crate) fn routes(user_state: UserState) -> OpenApiRouter {
  let user_routers = OpenApiRouter::new()
    .routes(routes!(list_users, new_user))
    .routes(routes!(get_user, update_user, delete_user))
    .with_state(user_state);
  OpenApiRouter::new().nest("/user", user_routers)
}
