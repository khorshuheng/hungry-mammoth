use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
  pub host: String,
  pub app_port: u16,
  pub metrics_port: u16,
}

impl ServerConfig {
  pub fn app_listener_address(&self) -> String {
    format!("{}:{}", self.host, self.app_port)
  }

  pub fn metrics_listener_address(&self) -> String {
    format!("{}:{}", self.host, self.metrics_port)
  }
}

#[derive(Debug, Deserialize)]
pub struct MetricsConfig {
  pub http_requests_latency_buckets: Vec<f64>,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
  pub host: String,
  pub port: u16,
  pub user: String,
  pub password: String,
  pub database: String,
  pub max_connections: u32,
  pub acquire_timeout: u64,
}

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
  pub secret: String,
  pub refresh_secret: String,
  pub token_expiry: u64,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AppConfig {
  pub server: ServerConfig,
  pub metrics: MetricsConfig,
  pub database: DatabaseConfig,
  pub auth: AuthConfig,
}

impl AppConfig {
  pub fn new() -> Result<Self, ConfigError> {
    let s = Config::builder()
      .add_source(File::with_name("config/default.toml"))
      .add_source(File::with_name("config/override.toml").required(false))
      .add_source(Environment::with_prefix("HM").separator("__"))
      .build()?;
    s.try_deserialize()
  }
}
