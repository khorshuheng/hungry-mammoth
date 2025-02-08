use async_trait::async_trait;
use sqlx::PgPool;

use crate::dto::user::{NewUserParameters, UpdateUserParameters, UserProfile};

use super::error::RepositoryError;

#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepository: Send + Sync {
  async fn list_users(&self) -> Result<Vec<UserProfile>, RepositoryError>;
  async fn get_user(&self, user_id: i32) -> Result<Option<UserProfile>, RepositoryError>;
  async fn new_user(&self, params: NewUserParameters) -> Result<UserProfile, RepositoryError>;
  async fn update_user(
    &self,
    user_id: i32,
    params: UpdateUserParameters,
  ) -> Result<(), RepositoryError>;
  async fn delete_user(&self, user_id: i32) -> Result<(), RepositoryError>;
}

#[derive(Clone)]
pub struct UserPostgresRepository {
  pool: PgPool,
}

impl UserPostgresRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl UserRepository for UserPostgresRepository {
  async fn list_users(&self) -> Result<Vec<UserProfile>, RepositoryError> {
    sqlx::query_as!(
      UserProfile,
      r#"
      SELECT id, email
      FROM user_profile
      "#
    )
    .fetch_all(&self.pool)
    .await
    .map_err(|err| err.into())
  }

  async fn get_user(&self, user_id: i32) -> Result<Option<UserProfile>, RepositoryError> {
    sqlx::query_as!(
      UserProfile,
      r#"
      SELECT id, email
      FROM user_profile
      WHERE id = $1
      "#,
      user_id
    )
    .fetch_optional(&self.pool)
    .await
    .map_err(|err| err.into())
  }

  async fn new_user(&self, params: NewUserParameters) -> Result<UserProfile, RepositoryError> {
    sqlx::query_as!(
      UserProfile,
      r#"
      INSERT INTO user_profile (email, password)
      VALUES ($1, $2)
      RETURNING id, email
      "#,
      params.email,
      params.password,
    )
    .fetch_one(&self.pool)
    .await
    .map_err(|err| err.into())
  }

  async fn update_user(
    &self,
    user_id: i32,
    params: UpdateUserParameters,
  ) -> Result<(), RepositoryError> {
    let updated_row_count = sqlx::query!(
      r#"
      UPDATE user_profile
      SET email = $1
      WHERE id = $2
      "#,
      params.email,
      user_id
    )
    .execute(&self.pool)
    .await?;
    if updated_row_count.rows_affected() == 0 {
      Err(RepositoryError::RowNotFound)
    } else {
      Ok(())
    }
  }

  async fn delete_user(&self, user_id: i32) -> Result<(), RepositoryError> {
    sqlx::query!(
      r#"
      DELETE FROM user_profile
      WHERE id = $1
      "#,
      user_id
    )
    .execute(&self.pool)
    .await
    .map(|_| ())
    .map_err(|err| err.into())
  }
}
