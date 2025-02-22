use crate::service::{auth::TokenService, user::UserService};

#[derive(Clone)]
pub struct AuthState {
  pub token_service: TokenService,
  pub user_service: UserService,
}
