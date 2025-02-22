use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::dto::user::*;

use super::error::RepositoryError;

#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepository: Send + Sync {
  async fn list_users(&self) -> Result<Vec<UserProfile>, RepositoryError>;
  async fn get_user_by_uuid(&self, user_uuid: Uuid)
    -> Result<Option<UserProfile>, RepositoryError>;
  async fn get_user_by_email(&self, email: &str) -> Result<Option<UserProfile>, RepositoryError>;
  async fn new_user(&self, params: NewUserProfile) -> Result<UserProfile, RepositoryError>;
  async fn update_user_by_uuid(
    &self,
    user_uuid: Uuid,
    params: UserProfileChange,
  ) -> Result<(), RepositoryError>;
  async fn delete_user_by_uuid(&self, user_uuid: Uuid) -> Result<(), RepositoryError>;
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
      SELECT uuid, email
      FROM user_profile
      "#
    )
    .fetch_all(&self.pool)
    .await
    .map_err(|err| err.into())
  }

  async fn get_user_by_uuid(
    &self,
    user_uuid: Uuid,
  ) -> Result<Option<UserProfile>, RepositoryError> {
    sqlx::query_as!(
      UserProfile,
      r#"
      SELECT uuid, email
      FROM user_profile
      WHERE uuid = $1
      "#,
      user_uuid
    )
    .fetch_optional(&self.pool)
    .await
    .map_err(|err| err.into())
  }

  async fn get_user_by_email(&self, email: &str) -> Result<Option<UserProfile>, RepositoryError> {
    sqlx::query_as!(
      UserProfile,
      r#"
        SELECT uuid, email
        FROM user_profile
        WHERE email = $1
        "#,
      email
    )
    .fetch_optional(&self.pool)
    .await
    .map_err(|err| err.into())
  }

  async fn new_user(&self, params: NewUserProfile) -> Result<UserProfile, RepositoryError> {
    sqlx::query_as!(
      UserProfile,
      r#"
      INSERT INTO user_profile (email, password_hash)
      VALUES ($1, $2)
      RETURNING uuid, email
      "#,
      params.email,
      params.password_hash,
    )
    .fetch_one(&self.pool)
    .await
    .map_err(|err| err.into())
  }

  async fn update_user_by_uuid(
    &self,
    user_uuid: Uuid,
    params: UserProfileChange,
  ) -> Result<(), RepositoryError> {
    let updated_row_count = sqlx::query!(
      r#"
      UPDATE user_profile
      SET email = $1
      WHERE uuid = $2
      "#,
      params.email,
      user_uuid
    )
    .execute(&self.pool)
    .await?;
    if updated_row_count.rows_affected() == 0 {
      Err(RepositoryError::RowNotFound)
    } else {
      Ok(())
    }
  }

  async fn delete_user_by_uuid(&self, user_uuid: Uuid) -> Result<(), RepositoryError> {
    sqlx::query!(
      r#"
      DELETE FROM user_profile
      WHERE uuid = $1
      "#,
      user_uuid
    )
    .execute(&self.pool)
    .await
    .map(|_| ())
    .map_err(|err| err.into())
  }
}
