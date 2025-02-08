use axum::{
  routing::{delete, get, post, put},
  Router,
};

use crate::{handler::user::*, state::user::UserState};

pub(crate) fn routes(user_state: UserState) -> Router {
  let user_routers = Router::new()
    .route("/", get(list_users))
    .route("/", post(new_user))
    .route("/{user_id}", get(get_user))
    .route("/{user_id}", put(update_user))
    .route("/{user_id}", delete(delete_user))
    .with_state(user_state);
  Router::new().nest("/users", user_routers)
}
