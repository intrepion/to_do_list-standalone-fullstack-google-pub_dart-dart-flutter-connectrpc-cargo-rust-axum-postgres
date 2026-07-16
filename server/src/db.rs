// Database connection management

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;

use crate::config::Config;
use crate::error::AppError;

/// Database connection pool type
pub type DbPool = PgPool;

/// Shared database state for Axum
#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<DbPool>,
}

impl AppState {
    /// Create a new AppState with a database connection pool
    pub async fn new(config: &Config) -> Result<Self, AppError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database_url)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        // Test the connection by running a simple query
        sqlx::query_scalar::<_, i64>("SELECT 1")
            .fetch_one(&pool)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        tracing::info!("Database connection pool established");

        Ok(Self {
            pool: Arc::new(pool),
        })
    }
}

/// Get a database connection from the pool
pub async fn get_connection(pool: &DbPool) -> Result<sqlx::pool::PoolConnection<sqlx::Postgres>, AppError> {
    pool.acquire()
        .await
        .map_err(|e| AppError::DatabaseError(e))
}

/// Execute a simple query to test database connectivity
pub async fn test_connection(pool: &DbPool) -> Result<i64, AppError> {
    sqlx::query_scalar::<_, i64>("SELECT 1")
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))
}
