use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
  #[error("sql execution failure: {0}")]
  SqlExecutionError(String),
  #[error("row not found")]
  RowNotFound,
  #[error("unable to acquire connection")]
  ConnectionTimeOut,
}

impl From<sqlx::Error> for RepositoryError {
  fn from(error: sqlx::Error) -> Self {
    match error {
      sqlx::Error::RowNotFound => RepositoryError::RowNotFound,
      sqlx::Error::PoolTimedOut => RepositoryError::ConnectionTimeOut,
      _ => RepositoryError::SqlExecutionError(error.to_string()),
    }
  }
}
