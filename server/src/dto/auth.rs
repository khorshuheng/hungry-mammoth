use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct GetTokenParameters {
  #[validate(email(message = "invalid email format"))]
  pub email: String,
  pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct AuthTokenResponse {
  pub token: String,
}
