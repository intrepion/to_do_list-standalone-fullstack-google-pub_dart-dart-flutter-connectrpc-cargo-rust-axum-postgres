// HTTP route handlers

use axum::{extract::Extension, Json};
use serde_json::json;
use std::sync::Arc;

use crate::db::{test_connection, DbPool};
use crate::error::AppError;

/// Health check endpoint
pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

/// Database connection health check
/// Tests the database connection by executing a simple query
pub async fn db_health_check(
    Extension(pool): Extension<Arc<DbPool>>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Test the database connection
    let result = test_connection(&pool).await?;
    
    if result == 1 {
        Ok(Json(json!({
            "status": "ok",
            "message": "Database connection successful",
            "database": "PostgreSQL"
        })))
    } else {
        Err(AppError::InternalServerError("Unexpected database response".to_string()))
    }
}
