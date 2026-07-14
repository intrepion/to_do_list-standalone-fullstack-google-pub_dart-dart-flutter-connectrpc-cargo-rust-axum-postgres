//! Build script for protobuf generation
//
//! This project uses Buf (https://buf.build/) for protocol buffer code generation.
//! Generated Rust code is in src/gen/ and is committed to the repository.
//!
//! To regenerate code:
//!   1. Install buf: brew install bufbuild/buf/buf
//!   2. Install plugins: buf plugin install
//!   3. Generate code: buf generate --template ../buf.gen.yaml ../proto
//!   4. Or use: make gen
//!
//! The build script watches the proto directory and will trigger a rebuild
//! if proto files change, but the actual code generation must be done manually
//! or via make commands.

fn main() {
    // Watch proto directory for changes to trigger cargo rebuild
    println!("cargo:rerun-if-changed=../proto/");
    
    // Generated files are in src/gen/
    println!("cargo:rerun-if-changed=src/gen/");
}
