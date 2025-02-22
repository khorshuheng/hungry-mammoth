use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
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

#[derive(Serialize)]
pub struct TokenClaims {
  pub sub: Uuid,
  pub email: String,
  pub iat: i64,
  pub exp: i64,
}
