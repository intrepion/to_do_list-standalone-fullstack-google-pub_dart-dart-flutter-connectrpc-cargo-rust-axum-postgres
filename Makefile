# Makefile for protocol buffer code generation

.PHONY: gen gen-rust gen-dart clean

# Generate all code (Rust + Dart)
gen: gen-rust gen-dart

# Generate Rust code
gen-rust:
	buf generate --template buf.gen.yaml proto

# Generate Dart code  
gen-dart:
	buf generate --template buf.gen.yaml proto

# Clean generated files
clean:
	rm -rf server/src/gen
	rm -rf client/lib/src/gen

# Full rebuild
rebuild: clean gen
