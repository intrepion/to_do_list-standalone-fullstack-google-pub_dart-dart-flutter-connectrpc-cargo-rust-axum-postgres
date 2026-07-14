//! HTTP server module using Axum

use crate::config::AppConfig;
use anyhow::Result;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_postgres::NoTls;

mod routes;

use routes::todo_routes;

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl AppState {
    /// Create a new application state
    pub async fn new(config: Arc<AppConfig>) -> Result<Self> {
        // Create database connection pool
        let mgr = PostgresConnectionManager::new_from_stringlike(
            &config.database_url,
            NoTls,
        )?;

        let db_pool = Pool::builder()
            .max_size(config.db_pool_max_size)
            .build(mgr)
            .await?;

        Ok(Self { config, db_pool })
    }
}

/// Create and run the HTTP server
pub async fn run_server(config: Arc<AppConfig>) -> Result<()> {
    // Create application state
    let state = Arc::new(AppState::new(config.clone()).await?);

    // Build the router
    let app = Router::new()
        // Health check endpoint
        .route("/health", axum::routing::get(|| async { "OK" }))
        // To-Do routes
        .nest("/api/v1/todos", todo_routes());

    // Add shared state
    let app = app.with_state(state);

    // Start the server
    let addr: SocketAddr = config.server_addr.parse()?;
    log::info!("Server listening on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(&addr).await?,
        app
    )
    .await?;

    Ok(())
}
