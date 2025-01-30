use axum::{
  routing::{delete, get, post, put},
  Router,
};

use crate::handler::user::*;

pub(crate) fn routes() -> Router {
  let user_routers = Router::new()
    .route("/", get(list_users))
    .route("/", post(new_user))
    .route("/{user_id}", get(get_user))
    .route("/{user_id}", put(update_user))
    .route("/{user_id}", delete(delete_user));
  Router::new().nest("/user", user_routers)
}
