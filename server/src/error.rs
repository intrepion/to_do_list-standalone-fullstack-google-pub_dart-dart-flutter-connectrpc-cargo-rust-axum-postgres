// Custom error types for the application

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    // Database errors
    DatabaseError(sqlx::Error),
    
    // Configuration errors
    ConfigError(String),
    
    // Authentication errors
    AuthError(String),
    
    // Not found errors
    NotFound(String),
    
    // Validation errors
    ValidationError(String),
    
    // Internal server errors
    InternalServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::ConfigError(e) => write!(f, "Configuration error: {}", e),
            AppError::AuthError(e) => write!(f, "Authentication error: {}", e),
            AppError::NotFound(e) => write!(f, "Not found: {}", e),
            AppError::ValidationError(e) => write!(f, "Validation error: {}", e),
            AppError::InternalServerError(e) => write!(f, "Internal server error: {}", e),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::InternalServerError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AuthError(_) => StatusCode::UNAUTHORIZED,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        
        (status, self.to_string()).into_response()
    }
}
