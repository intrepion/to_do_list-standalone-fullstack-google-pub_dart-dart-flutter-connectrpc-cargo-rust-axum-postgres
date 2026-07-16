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
    
    // CORS configuration
    pub cors_allowed_origins: Vec<String>,
    pub cors_allowed_methods: Vec<String>,
    pub cors_allowed_headers: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_host: "0.0.0.0".to_string(),
            server_port: 3000,
            database_url: "".to_string(),
            google_client_id: "".to_string(),
            google_client_secret: "".to_string(),
            google_redirect_uri: "http://localhost:3000/auth/callback".to_string(),
            jwt_secret: "".to_string(),
            jwt_expiration_days: 7,
            cors_allowed_origins: vec!["http://localhost:*".to_string()],
            cors_allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "OPTIONS".to_string(),
            ],
            cors_allowed_headers: vec![
                "Content-Type".to_string(),
                "Authorization".to_string(),
                "Accept".to_string(),
            ],
        }
    }
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, String> {
        dotenv::dotenv().ok();

        let mut config = Self::default();

        config.server_host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| config.server_host);
        config.server_port = env::var("SERVER_PORT")
            .map(|s| s.parse().unwrap_or(config.server_port))
            .unwrap_or(config.server_port);
        config.database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| panic!("DATABASE_URL must be set"));
        config.google_client_id = env::var("GOOGLE_CLIENT_ID")
            .unwrap_or_else(|_| panic!("GOOGLE_CLIENT_ID must be set"));
        config.google_client_secret = env::var("GOOGLE_CLIENT_SECRET")
            .unwrap_or_else(|_| panic!("GOOGLE_CLIENT_SECRET must be set"));
        config.google_redirect_uri = env::var("GOOGLE_REDIRECT_URI")
            .unwrap_or_else(|_| config.google_redirect_uri);
        config.jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| panic!("JWT_SECRET must be set"));
        config.jwt_expiration_days = env::var("JWT_EXPIRATION_DAYS")
            .map(|s| s.parse().unwrap_or(config.jwt_expiration_days))
            .unwrap_or(config.jwt_expiration_days);
        
        // CORS configuration (optional, uses defaults if not set)
        if let Ok(origins) = env::var("CORS_ALLOWED_ORIGINS") {
            config.cors_allowed_origins = origins
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }
        if let Ok(methods) = env::var("CORS_ALLOWED_METHODS") {
            config.cors_allowed_methods = methods
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }
        if let Ok(headers) = env::var("CORS_ALLOWED_HEADERS") {
            config.cors_allowed_headers = headers
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        Ok(config)
    }
}
