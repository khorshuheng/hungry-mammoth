use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct UserProfile {
  pub id: i32,
  pub email: String,
}

#[derive(Serialize)]
pub struct ListUsersResponse {
  pub users: Vec<UserProfile>,
}

#[derive(Deserialize, Validate)]
pub struct NewUserParameters {
  #[validate(email(message = "invalid email format"))]
  pub email: String,
  #[validate(length(min = 6, max = 20, message = "must be between 6 and 20 characters"))]
  pub password: String,
}

#[derive(Serialize)]
pub struct NewUserResponse {
  pub user: UserProfile,
}

#[derive(Serialize)]
pub struct GetUserResponse {
  pub user: UserProfile,
}

#[derive(Deserialize, Validate)]
pub struct UpdateUserParameters {
  #[validate(email(message = "invalid email format"))]
  pub email: String,
  #[validate(length(min = 6, max = 20, message = "must be between 6 and 20 characters"))]
  pub password: String,
}
