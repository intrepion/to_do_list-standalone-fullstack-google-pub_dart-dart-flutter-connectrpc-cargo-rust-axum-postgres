// Build script for protocol buffer generation
// This will generate Rust code from .proto files using connectrpc

use std::env;
use std::path::Path;

fn main() {
    // Get the directory where proto files are located
    let proto_dir = "proto";
    
    // Get the output directory for generated files
    let out_dir = env::var_os("OUT_DIR").unwrap();
    
    // This will be populated when we have .proto files
    // TODO: Add protoc compilation for todo.proto
    
    println!("cargo:rerun-if-changed={}", Path::new(proto_dir).display());
}