use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct ListUsersResponse {}

#[derive(Deserialize, Validate)]
pub struct NewUserParameters {
  #[validate(email(message = "invalid email format"))]
  pub email: String,
  #[validate(length(min = 6, max = 20, message = "must be between 6 and 20 characters"))]
  pub password: String,
}

#[derive(Serialize)]
pub struct NewUserResponse {}

#[derive(Serialize)]
pub struct GetUserResponse {}

#[derive(Deserialize, Validate)]
pub struct UpdateUserParameters {}

#[derive(Serialize)]
pub struct UpdateUserResponse {}

#[derive(Serialize)]
pub struct DeleteUserResponse {}
