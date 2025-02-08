use std::sync::Arc;

use crate::{config::AppConfig, repository::user::UserPostgresRepository};

use super::{
  database::{self, migrate},
  user::UserState,
};

pub struct AppState {
  pub user_state: UserState,
}

impl AppState {
  pub async fn new(app_config: &AppConfig) -> Self {
    let pool = database::create_pg_pool(&app_config.database)
      .await
      .expect("failed to create database connection pool");
    migrate(&pool).await.expect("failed to migrate database");
    let user_repository = Arc::new(UserPostgresRepository::new(pool));
    let user_service = crate::service::user::UserService::new(user_repository);
    let user_state = crate::state::user::UserState { user_service };
    Self { user_state }
  }
}
