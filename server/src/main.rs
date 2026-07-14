// Todo Server Main Entry Point
// This is the entry point for the Rust server using Axum and ConnectRPC

mod config;
mod handlers;
mod middleware;
mod models;
mod repository;
mod routes;
mod services;
mod utils;

use config::AppConfig;
use routes::create_router;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    // Load configuration
    let config = AppConfig::from_env().expect("Failed to load configuration");

    tracing::info!("Starting todo server...");
    tracing::info!("Configuration: {:?}", config);

    // Create router
    let app = create_router();

    // Start server
    let addr = SocketAddr::from((config.host, config.port));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");
    axum::serve(listener, app).await.expect("Server failed to start");
}
