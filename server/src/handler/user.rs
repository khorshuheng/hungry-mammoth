use axum::{extract::State, http::StatusCode, response::NoContent, Json};

use crate::{
  dto::{
    user::*,
    wrapper::{ApiError, ApiSuccess},
  },
  state::user::UserState,
};

pub async fn list_users(
  State(user_state): State<UserState>,
) -> Result<ApiSuccess<ListUsersResponse>, ApiError> {
  let users = user_state.user_service.list_users().await?;
  Ok(ApiSuccess {
    status: StatusCode::OK,
    data: ListUsersResponse { users },
  })
}

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

pub async fn update_user(
  State(user_state): State<UserState>,
  Json(params): Json<UpdateUserParameters>,
) -> Result<NoContent, ApiError> {
  user_state.user_service.update_user(0, params).await?;
  Ok(NoContent)
}

pub async fn get_user(
  State(user_state): State<UserState>,
) -> Result<ApiSuccess<GetUserResponse>, ApiError> {
  let user = user_state.user_service.get_user(0).await?;
  Ok(ApiSuccess {
    status: StatusCode::OK,
    data: GetUserResponse { user },
  })
}

pub async fn delete_user(State(user_state): State<UserState>) -> Result<NoContent, ApiError> {
  user_state.user_service.delete_user(0).await?;
  Ok(NoContent)
}
