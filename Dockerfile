# Dockerfile for Rust Todo Server
FROM rust:1.96-slim as builder

# Install required dependencies for building
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Install buf (for ConnectRPC code generation)
RUN curl -sSL https://github.com/bufbuild/buf/releases/download/v1.71.0/buf-Linux-x86_64 -o /usr/local/bin/buf \
    && chmod +x /usr/local/bin/buf

# Set up working directory
WORKDIR /app

# Copy all files
COPY . .

# Generate Rust code from proto files
RUN buf generate proto/todo.proto

# Build the Rust application
RUN cargo build --release

# Create runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
WORKDIR /app
COPY --from=builder /app/target/release/todo_server .

# Copy proto files (for reference)
COPY --from=builder /app/proto ./proto

# Set environment variables
ENV RUST_LOG=info

# Expose port
EXPOSE 8080

# Set entrypoint
ENTRYPOINT ["./todo_server"]
CMD []