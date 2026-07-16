// HTTP route handlers

use axum::Json;
use serde_json::json;

/// Health check endpoint
pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

/// Database connection health check
pub async fn db_health_check() -> Json<serde_json::Value> {
    // This will be enhanced in MVP 1 to actually check the database connection
    Json(json!({
        "status": "ok",
        "message": "Database connection check placeholder"
    }))
}
