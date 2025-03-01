use axum::{
  extract::{Path, State},
  http::StatusCode,
  response::NoContent,
  Json,
};
use uuid::Uuid;

use crate::{
  dto::{
    user::*,
    wrapper::{ApiError, ApiSuccess},
  },
  state::user::UserState,
};

static USER_TAG: &str = "User";

#[utoipa::path(
  tag = USER_TAG,
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
  tag = USER_TAG,
  post,
  path = "",
  request_body = NewUserParameters,
  responses(
    (status = 201, description = "New user created", body = NewUserResponse),
    (status = 409, description = "Duplicated user")
  )
)]
pub async fn new_user(
  State(user_state): State<UserState>,
  Json(params): Json<NewUserParameters>,
) -> Result<ApiSuccess<NewUserResponse>, ApiError> {
  let user = user_state.user_service.new_user(params).await?;
  Ok(ApiSuccess {
    status: StatusCode::CREATED,
    data: NewUserResponse { user },
  })
}

#[utoipa::path(
  tag = USER_TAG,
  put,
  path = "/{user_uuid}",
  params(
    ("user_uuid" = Uuid, Path, description = "User UUID")
  ),
  responses(
    (status = 204, description = "User updated"),
    (status = 404, description = "User not found"),
  )
)]
pub async fn update_user(
  State(user_state): State<UserState>,
  Path(user_uuid): Path<Uuid>,
  Json(params): Json<UpdateUserParameters>,
) -> Result<NoContent, ApiError> {
  user_state
    .user_service
    .update_user(user_uuid, params)
    .await?;
  Ok(NoContent)
}

#[utoipa::path(
  tag = USER_TAG,
  get,
  path = "/{user_uuid}",
  params(
      ("user_uuid" = Uuid, Path, description = "User UUID")
    ),
  responses(
    (status = 204, description = "Successfully get user profile"),
    (status = 404, description = "User not found"),
  )
)]
pub async fn get_user(
  State(user_state): State<UserState>,
  Path(user_uuid): Path<Uuid>,
) -> Result<ApiSuccess<GetUserResponse>, ApiError> {
  let user = user_state.user_service.get_user(user_uuid).await?;
  Ok(ApiSuccess {
    status: StatusCode::OK,
    data: GetUserResponse { user },
  })
}

#[utoipa::path(
  tag = USER_TAG,
  delete,
  path = "/{user_uuid}",
  params(
      ("user_uuid" = Uuid, Path, description = "User UUID")
    ),
  responses(
    (status = 204, description = "User deleted"),
  )
)]
pub async fn delete_user(
  State(user_state): State<UserState>,
  Path(user_uuid): Path<Uuid>,
) -> Result<NoContent, ApiError> {
  user_state.user_service.delete_user(user_uuid).await?;
  Ok(NoContent)
}
