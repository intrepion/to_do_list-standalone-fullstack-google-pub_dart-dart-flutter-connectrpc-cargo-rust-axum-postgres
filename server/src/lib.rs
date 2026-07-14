//! To-Do List Server Library
//!
//! This library provides the core functionality for the to-do list server,
//! including configuration, database models, and service implementations.

pub mod config;
pub mod models;
pub mod server;
pub mod services;

// Include generated protobuf code
pub mod gen {
    include!("gen/_connectrpc.rs");
}
