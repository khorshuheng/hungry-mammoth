use std::sync::Arc;

use crate::{
  dto::user::*,
  repository::{error::RepositoryError, user::UserRepository},
};

use super::error::ServiceError;

#[derive(Clone)]
pub struct UserService {
  repository: Arc<dyn UserRepository>,
}

impl UserService {
  pub fn new(repository: Arc<dyn UserRepository>) -> Self {
    Self { repository }
  }

  pub async fn list_users(&self) -> Result<Vec<UserProfile>, ServiceError> {
    match self.repository.list_users().await {
      Ok(users) => Ok(users),
      Err(err) => Err(ServiceError::InternalError(err.to_string())),
    }
  }

  pub async fn get_user(&self, user_id: i32) -> Result<UserProfile, ServiceError> {
    match self.repository.get_user(user_id).await {
      Ok(Some(user)) => Ok(user),
      Ok(None) => Err(ServiceError::EntityNotFound),
      Err(err) => Err(ServiceError::InternalError(format!(
        "unable to get user: {}",
        err
      ))),
    }
  }

  pub async fn new_user(&self, params: NewUserParameters) -> Result<UserProfile, ServiceError> {
    match self.repository.new_user(params).await {
      Ok(user) => Ok(user),
      Err(err) => Err(ServiceError::InternalError(format!(
        "unable to create user: {}",
        err
      ))),
    }
  }

  pub async fn update_user(
    &self,
    user_id: i32,
    params: UpdateUserParameters,
  ) -> Result<(), ServiceError> {
    match self.repository.update_user(user_id, params).await {
      Ok(_) => Ok(()),
      Err(RepositoryError::RowNotFound) => Err(ServiceError::EntityNotFound),
      Err(err) => Err(ServiceError::InternalError(format!(
        "unable to update user: {}",
        err
      ))),
    }
  }

  pub async fn delete_user(&self, user_id: i32) -> Result<(), ServiceError> {
    match self.repository.delete_user(user_id).await {
      Ok(_) => Ok(()),
      Err(err) => Err(ServiceError::InternalError(format!(
        "unable to delete user: {}",
        err
      ))),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::repository::user::MockUserRepository;

  use super::*;

  #[tokio::test]
  async fn test_list_users() {
    let users = vec![UserProfile {
      id: 1,
      email: "user@domain.com".to_string(),
    }];
    let mut mock = MockUserRepository::new();
    let cloned_users = users.clone();
    mock
      .expect_list_users()
      .returning(move || Ok(cloned_users.clone()));
    let user_service = UserService::new(Arc::new(mock));
    let result = user_service
      .list_users()
      .await
      .expect("failed to list users");
    assert_eq!(result, users);
  }

  #[tokio::test]
  async fn test_get_user() {
    let user = UserProfile {
      id: 1,
      email: "user@domain.com".to_string(),
    };
    let mut mock = MockUserRepository::new();
    let cloned_user = user.clone();
    mock
      .expect_get_user()
      .returning(move |_| Ok(Some(cloned_user.clone())));
    let result = UserService::new(Arc::new(mock))
      .get_user(1)
      .await
      .expect("failed to get user");
    assert_eq!(user, result)
  }

  #[tokio::test]
  async fn test_get_non_existant_user() {
    let mut mock = MockUserRepository::new();
    mock.expect_get_user().returning(move |_| Ok(None));
    let result = UserService::new(Arc::new(mock))
      .get_user(1)
      .await
      .expect_err("expected to fail");
    assert_eq!(ServiceError::EntityNotFound, result)
  }
}
