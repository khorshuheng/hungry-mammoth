use axum::{http::StatusCode, Json};

use crate::dto::{
  auth::{AuthTokenResponse, GetTokenParameters},
  wrapper::{ApiError, ApiSuccess},
};

static AUTH_TAG: &str = "Auth";

#[utoipa::path(
  tag = AUTH_TAG,
  post,
  path = "/token",
  request_body = GetTokenParameters,
  responses(
    (status = 200, description = "Get access token", body = AuthTokenResponse),
    (status = 401, description = "Invalid credential")
  )
)]
pub async fn get_token(
  Json(_): Json<GetTokenParameters>,
) -> Result<ApiSuccess<AuthTokenResponse>, ApiError> {
  Ok(ApiSuccess {
    status: StatusCode::OK,
    data: AuthTokenResponse {
      token: "abcde".to_string(),
    },
  })
}
