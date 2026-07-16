// Configuration management using environment variables

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    // Server configuration
    pub server_host: String,
    pub server_port: u16,
    
    // Database configuration
    pub database_url: String,
    
    // Google OAuth 2.0 configuration
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    
    // JWT configuration
    pub jwt_secret: String,
    pub jwt_expiration_days: i64,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, String> {
        dotenv::dotenv().ok();

        Ok(Self {
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .map(|s| s.parse().unwrap_or(3000))
                .unwrap_or(3000),
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            google_client_id: env::var("GOOGLE_CLIENT_ID")
                .expect("GOOGLE_CLIENT_ID must be set"),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET")
                .expect("GOOGLE_CLIENT_SECRET must be set"),
            google_redirect_uri: env::var("GOOGLE_REDIRECT_URI")
                .unwrap_or_else(|_| "http://localhost:3000/auth/callback".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            jwt_expiration_days: env::var("JWT_EXPIRATION_DAYS")
                .map(|s| s.parse().unwrap_or(7))
                .unwrap_or(7),
        })
    }
}
