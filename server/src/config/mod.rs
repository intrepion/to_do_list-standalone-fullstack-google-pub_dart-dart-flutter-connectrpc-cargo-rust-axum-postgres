//! Application configuration

use anyhow::Result;
use serde::Deserialize;
use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    /// Server address (e.g., "0.0.0.0:8080")
    pub server_addr: String,
    /// PostgreSQL connection string
    pub database_url: String,
    /// Maximum database connection pool size
    pub db_pool_max_size: u32,
    /// Environment (development, staging, production)
    pub environment: String,
    /// Log level (debug, info, warn, error)
    pub log_level: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_addr: "0.0.0.0:8080".to_string(),
            database_url: "postgres://user:password@localhost:5432/to_do_list".to_string(),
            db_pool_max_size: 10,
            environment: "development".to_string(),
            log_level: "info".to_string(),
        }
    }
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv()?;

        let config = Self {
            server_addr: env::var("SERVER_ADDR")
                .unwrap_or_else(|_| Self::default().server_addr),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| Self::default().database_url),
            db_pool_max_size: env::var("DB_POOL_MAX_SIZE")
                .map(|s| s.parse().unwrap_or(Self::default().db_pool_max_size))
                .unwrap_or(Self::default().db_pool_max_size),
            environment: env::var("ENVIRONMENT")
                .unwrap_or_else(|_| Self::default().environment),
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| Self::default().log_level),
        };

        Ok(config)
    }
}
