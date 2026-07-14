//! Build script for protobuf generation
//!
//! This project uses connectrpc-build for protocol buffer code generation.
//! Generated Rust code is in src/gen/ and is committed to the repository.
//!
//! To regenerate code manually:
//!   1. Install buf: brew install bufbuild/buf/buf
//!   2. Use: buf generate --template ../buf.gen.yaml ../proto
//!   3. Or use: make gen
//!
//! The build script uses connectrpc-build to generate code during cargo build.

fn main() {
    // Generate code for all proto files in the ../proto directory
    connectrpc_build::Config::new()
        .files(&[
            "../proto/todo/v1/todo.proto",
            "../proto/auth/v1/auth.proto",
        ])
        .includes(&["../proto/"])
        .out_dir("src/gen/")
        .include_file("_connectrpc.rs")
        .compile()
        .expect("Failed to generate protobuf code");
}
