// To-Do List Server
// Main entry point for the axum + connectrpc server

mod config;
mod db;
mod error;
mod routes;

use axum::{Extension, Router};
use config::Config;
use db::AppState;
use routes::{db_health_check, health_check};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,todo_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    tracing::info!("Configuration loaded successfully");

    // Initialize database connection pool
    let state = AppState::new(&config).await.expect("Failed to initialize database");
    let pool = state.pool.clone();

    // Build router with routes
    let app = Router::new()
        // Health check endpoint
        .route("/health", axum::routing::get(health_check))
        // Database connection test endpoint
        .route("/health/db", axum::routing::get(db_health_check))
        // Add database pool to extensions for routes that need it
        .layer(Extension(pool));

    // Parse host and create socket address
    let ip = IpAddr::from_str(&config.server_host).expect("Invalid server host");
    let addr = SocketAddr::new(ip, config.server_port);
    tracing::info!("Server starting on {}", addr);

    // Start server using axum's serve with hyper
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address"),
        app,
    )
    .await
    .expect("Server failed to start");
}
