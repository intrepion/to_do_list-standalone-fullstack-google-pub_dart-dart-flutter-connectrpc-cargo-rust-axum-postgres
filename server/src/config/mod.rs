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
    /// Google OAuth 2.0 client ID
    pub google_oauth_client_id: String,
    /// Google OAuth 2.0 client secret
    pub google_oauth_client_secret: String,
    /// Google OAuth 2.0 redirect URI
    pub google_oauth_redirect_uri: String,
    /// JWT signing secret key
    pub jwt_secret: String,
    /// JWT token expiration in hours (default: 24)
    pub jwt_expires_in_hours: u64,
    /// JWT issuer (optional)
    pub jwt_issuer: Option<String>,
    /// JWT audience (optional)
    pub jwt_audience: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_addr: "0.0.0.0:8080".to_string(),
            database_url: "postgres://user:password@localhost:5432/to_do_list".to_string(),
            db_pool_max_size: 10,
            environment: "development".to_string(),
            log_level: "info".to_string(),
            google_oauth_client_id: "".to_string(),
            google_oauth_client_secret: "".to_string(),
            google_oauth_redirect_uri: "http://localhost:8080/auth/callback".to_string(),
            jwt_secret: "change-me-in-production-use-a-strong-secret".to_string(),
            jwt_expires_in_hours: 24,
            jwt_issuer: None,
            jwt_audience: None,
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
            google_oauth_client_id: env::var("GOOGLE_OAUTH_CLIENT_ID")
                .unwrap_or_else(|_| Self::default().google_oauth_client_id),
            google_oauth_client_secret: env::var("GOOGLE_OAUTH_CLIENT_SECRET")
                .unwrap_or_else(|_| Self::default().google_oauth_client_secret),
            google_oauth_redirect_uri: env::var("GOOGLE_OAUTH_REDIRECT_URI")
                .unwrap_or_else(|_| Self::default().google_oauth_redirect_uri),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| Self::default().jwt_secret),
            jwt_expires_in_hours: env::var("JWT_EXPIRES_IN_HOURS")
                .map(|s| s.parse().unwrap_or(Self::default().jwt_expires_in_hours))
                .unwrap_or(Self::default().jwt_expires_in_hours),
            jwt_issuer: env::var("JWT_ISSUER").ok(),
            jwt_audience: env::var("JWT_AUDIENCE").ok(),
        };

        Ok(config)
    }
}
