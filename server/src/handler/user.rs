use crate::{
  dto::{
    user::*,
    wrapper::{ApiResult, ApiSuccess},
  },
  extractor::validation::ValidatedRequest,
};

pub async fn list_users() -> ApiResult<ListUsersResponse> {
  Ok(ApiSuccess::send(ListUsersResponse {}))
}

pub async fn new_user(
  ValidatedRequest(_params): ValidatedRequest<NewUserParameters>,
) -> ApiResult<NewUserResponse> {
  Ok(ApiSuccess::send(NewUserResponse {}))
}

pub async fn update_user(
  ValidatedRequest(_params): ValidatedRequest<UpdateUserParameters>,
) -> ApiResult<UpdateUserResponse> {
  Ok(ApiSuccess::send(UpdateUserResponse {}))
}

pub async fn get_user() -> ApiResult<GetUserResponse> {
  Ok(ApiSuccess::send(GetUserResponse {}))
}

pub async fn delete_user() -> ApiResult<DeleteUserResponse> {
  Ok(ApiSuccess::send(DeleteUserResponse {}))
}
