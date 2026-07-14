mod config;
mod models;
mod server;
mod services;

use anyhow::Result;
use config::AppConfig;
use server::run_server;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    pretty_env_logger::init();
    log::info!("Starting to-do list server");

    // Load configuration
    let config = Arc::new(AppConfig::from_env()?);
    log::info!("Configuration loaded: {:?}", config);

    // Run the server
    run_server(config).await?;

    Ok(())
}
