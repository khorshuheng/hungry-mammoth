use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::DatabaseConfig;

pub async fn create_pg_pool(database_config: &DatabaseConfig) -> sqlx::Result<sqlx::PgPool> {
  let pg_pool_options = sqlx::postgres::PgConnectOptions::new()
    .host(&database_config.host)
    .port(database_config.port)
    .username(&database_config.user)
    .password(&database_config.password)
    .database(&database_config.database);

  PgPoolOptions::new()
    .max_connections(database_config.max_connections)
    .acquire_timeout(Duration::from_secs(database_config.acquire_timeout))
    .connect_with(pg_pool_options)
    .await
}

pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
  sqlx::migrate!("./migrations")
    .set_ignore_missing(true)
    .run(pool)
    .await
}
