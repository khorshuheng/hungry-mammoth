use axum::{extract::State, http::StatusCode, Json};

use crate::{
  dto::{
    auth::{AuthTokenResponse, GetTokenParameters},
    wrapper::{ApiError, ApiSuccess},
  },
  state::auth::AuthState,
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
  State(auth_state): State<AuthState>,
  Json(params): Json<GetTokenParameters>,
) -> Result<ApiSuccess<AuthTokenResponse>, ApiError> {
  let user = auth_state
    .user_service
    .get_user_by_email(&params.email)
    .await?;
  let token = auth_state.token_service.generate_token(user)?;
  Ok(ApiSuccess {
    status: StatusCode::OK,
    data: AuthTokenResponse { token },
  })
}
