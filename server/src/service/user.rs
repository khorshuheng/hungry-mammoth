use std::sync::Arc;

use argon2::{
  password_hash::{rand_core::OsRng, SaltString},
  Argon2, PasswordHasher,
};
use uuid::Uuid;

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

  async fn hash_password(password: &str, salt: SaltString) -> String {
    let password = password.to_string();
    tokio::task::spawn_blocking(move || {
      let argon2 = Argon2::default();
      argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
    })
    .await
    .unwrap()
  }

  pub async fn list_users(&self) -> Result<Vec<UserProfile>, ServiceError> {
    match self.repository.list_users().await {
      Ok(users) => Ok(users),
      Err(err) => Err(ServiceError::InternalError(err.to_string())),
    }
  }

  pub async fn get_user(&self, user_uuid: Uuid) -> Result<UserProfile, ServiceError> {
    match self.repository.get_user_by_uuid(user_uuid).await {
      Ok(Some(user)) => Ok(user),
      Ok(None) => Err(ServiceError::EntityNotFound),
      Err(err) => Err(ServiceError::InternalError(format!(
        "unable to get user: {}",
        err
      ))),
    }
  }

  pub async fn get_user_by_email(&self, email: &str) -> Result<UserProfile, ServiceError> {
    match self.repository.get_user_by_email(email).await {
      Ok(Some(user)) => Ok(user),
      Ok(None) => Err(ServiceError::EntityNotFound),
      Err(err) => Err(ServiceError::InternalError(format!(
        "unable to get user: {}",
        err
      ))),
    }
  }

  pub async fn new_user(&self, params: NewUserParameters) -> Result<UserProfile, ServiceError> {
    let salt = SaltString::generate(&mut OsRng);
    let new_user_profile = NewUserProfile {
      email: params.email,
      password_hash: Self::hash_password(&params.password, salt.clone()).await,
    };
    match self.repository.new_user(new_user_profile).await {
      Ok(user) => Ok(user),
      Err(RepositoryError::UniqueConstraintViolation(err)) => Err(ServiceError::Conflict(err)),
      Err(err) => Err(ServiceError::InternalError(format!(
        "error creating new user: {}",
        err
      ))),
    }
  }

  pub async fn update_user(
    &self,
    user_uuid: Uuid,
    params: UpdateUserParameters,
  ) -> Result<(), ServiceError> {
    let password_hash = match params.password {
      Some(_) => {
        let salt = SaltString::generate(&mut OsRng);
        Some(Self::hash_password(&params.password.unwrap(), salt).await)
      },
      None => None,
    };

    let updated_user_profile = UserProfileChange {
      email: params.email,
      password_hash,
    };

    match self
      .repository
      .update_user_by_uuid(user_uuid, updated_user_profile)
      .await
    {
      Ok(_) => Ok(()),
      Err(RepositoryError::RowNotFound) => Err(ServiceError::EntityNotFound),
      Err(err) => Err(ServiceError::InternalError(format!(
        "unable to update user: {}",
        err
      ))),
    }
  }

  pub async fn delete_user(&self, user_uuid: Uuid) -> Result<(), ServiceError> {
    match self.repository.delete_user_by_uuid(user_uuid).await {
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
  use argon2::{PasswordHash, PasswordVerifier};

  use crate::repository::user::MockUserRepository;

  use super::*;

  #[tokio::test]
  async fn test_new_user() {
    let email = "new_user@domain.com".to_string();
    let password: String = "password".to_string();
    let mut mock = MockUserRepository::new();
    let inner_email = email.clone();
    let inner_password = password.clone();
    mock
      .expect_new_user()
      .withf(move |new_user_profile| {
        let argon2 = Argon2::default();
        // let salt = SaltString::from_b64(&new_user_profile.password_salt).expect("invalid salt");
        let password_hash =
          PasswordHash::new(&new_user_profile.password_hash).expect("invalid hash");
        argon2
          .verify_password(inner_password.as_bytes(), &password_hash)
          .is_ok()
      })
      .returning(move |_| {
        Ok(UserProfile {
          uuid: Uuid::new_v4(),
          email: inner_email.clone(),
        })
      });
    let user_service = UserService::new(Arc::new(mock));
    let params = NewUserParameters { email, password };
    user_service
      .new_user(params)
      .await
      .expect("failed to create new user");
  }

  #[tokio::test]
  async fn test_list_users() {
    let users = vec![UserProfile {
      uuid: Uuid::new_v4(),
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
    let user_uuid = Uuid::new_v4();
    let user = UserProfile {
      uuid: user_uuid,
      email: "user@domain.com".to_string(),
    };
    let mut mock = MockUserRepository::new();
    let cloned_user = user.clone();
    mock
      .expect_get_user_by_uuid()
      .returning(move |_| Ok(Some(cloned_user.clone())));
    let result = UserService::new(Arc::new(mock))
      .get_user(user_uuid)
      .await
      .expect("failed to get user");
    assert_eq!(user, result)
  }

  #[tokio::test]
  async fn test_get_non_existant_user() {
    let mut mock = MockUserRepository::new();
    mock.expect_get_user_by_uuid().returning(move |_| Ok(None));
    let result = UserService::new(Arc::new(mock))
      .get_user(Uuid::nil())
      .await
      .expect_err("expected to fail");
    assert_eq!(ServiceError::EntityNotFound, result)
  }
}
