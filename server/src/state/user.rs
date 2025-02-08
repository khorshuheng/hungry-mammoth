use crate::service::user::UserService;

#[derive(Clone)]
pub struct UserState {
  pub user_service: UserService,
}
