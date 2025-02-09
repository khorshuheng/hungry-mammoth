use axum::{extract::State, http::StatusCode, response::NoContent, Json};

use crate::{
  dto::{
    user::*,
    wrapper::{ApiError, ApiSuccess},
  },
  state::user::UserState,
};

#[utoipa::path(
  get,
  path = "",
  responses(
    (status = 200, description = "List user profile succeeded", body = ListUsersResponse)
  )
)]
pub async fn list_users(
  State(user_state): State<UserState>,
) -> Result<ApiSuccess<ListUsersResponse>, ApiError> {
  let users = user_state.user_service.list_users().await?;
  Ok(ApiSuccess {
    status: StatusCode::OK,
    data: ListUsersResponse { users },
  })
}

#[utoipa::path(
  post,
  path = "",
  request_body = NewUserParameters,
  responses(
    (status = 201, description = "New user created", body = NewUserResponse)
  )
)]
pub async fn new_user(
  State(user_state): State<UserState>,
  Json(params): Json<NewUserParameters>,
) -> Result<ApiSuccess<NewUserResponse>, ApiError> {
  let user = user_state.user_service.new_user(params).await?;
  Ok(ApiSuccess {
    status: StatusCode::ACCEPTED,
    data: NewUserResponse { user },
  })
}

#[utoipa::path(
  put,
  path = "/{user_id}",
  params(
    ("user_id" = i32, Path, description = "User ID")
  ),
  responses(
    (status = 204, description = "User updated"),
    (status = 404, description = "User not found"),
  )
)]
pub async fn update_user(
  State(user_state): State<UserState>,
  Json(params): Json<UpdateUserParameters>,
) -> Result<NoContent, ApiError> {
  user_state.user_service.update_user(0, params).await?;
  Ok(NoContent)
}

#[utoipa::path(
  get,
  path = "/{user_id}",
  params(
      ("user_id" = i32, Path, description = "User ID")
    ),
  responses(
    (status = 204, description = "Successfully get user profile"),
    (status = 404, description = "User not found"),
  )
)]
pub async fn get_user(
  State(user_state): State<UserState>,
) -> Result<ApiSuccess<GetUserResponse>, ApiError> {
  let user = user_state.user_service.get_user(0).await?;
  Ok(ApiSuccess {
    status: StatusCode::OK,
    data: GetUserResponse { user },
  })
}

#[utoipa::path(
  delete,
  path = "/{user_id}",
  params(
      ("user_id" = i32, Path, description = "User ID")
    ),
  responses(
    (status = 204, description = "User deleted"),
  )
)]
pub async fn delete_user(State(user_state): State<UserState>) -> Result<NoContent, ApiError> {
  user_state.user_service.delete_user(0).await?;
  Ok(NoContent)
}
