// Application Configuration
// Handles environment variables and application settings

use serde::Deserialize;
use std::net::IpAddr;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub host: IpAddr,
    pub port: u16,
    pub database_url: String,
    pub google_client_id: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, dotenvy::Error> {
        dotenvy::dotenv()?;
        
        Ok(Self {
            host: std::env::var("HOST")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(|| [127, 0, 0, 1].into()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(8080),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://user:password@localhost:5432/todo_db".to_string()),
            google_client_id: std::env::var("GOOGLE_CLIENT_ID")
                .expect("GOOGLE_CLIENT_ID must be set"),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "default-secret-change-me".to_string()),
            jwt_expiration_hours: std::env::var("JWT_EXPIRATION_HOURS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(24),
        })
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: [127, 0, 0, 1].into(),
            port: 8080,
            database_url: "postgres://user:password@localhost:5432/todo_db".to_string(),
            google_client_id: "".to_string(),
            jwt_secret: "default-secret-change-me".to_string(),
            jwt_expiration_hours: 24,
        }
    }
}