//! Build script for protobuf generation

fn main() {
    // This will generate Rust code from .proto files
    // Run with: cargo build
    println!("cargo:rerun-if-changed=src/protos/");

    // For now, we'll add proto generation when .proto files are added
    // prost_build::compile_protos(&["src/protos/todo.proto"], &["src/protos/"]).unwrap();
}
