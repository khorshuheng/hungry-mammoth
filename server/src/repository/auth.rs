use super::error::RepositoryError;

use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait AuthRepository: Send + Sync {
  async fn store_refresh_token(&self, refresh_token: String) -> Result<(), RepositoryError>;
  async fn refresh_token_match(&self, refresh_token: String) -> Result<bool, RepositoryError>;
}
