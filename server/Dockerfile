# Rust Server Dockerfile
# Multi-stage build for smaller final image

# Build stage
FROM rust:1.75-slim AS builder

# Install protobuf compiler
RUN apt-get update && apt-get install -y protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy all files
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/todo-server /app/todo-server

# Copy proto files (if needed at runtime)
COPY --from=builder /app/proto /app/proto

# Set environment variables
ENV RUST_LOG=info

# Expose port
EXPOSE 3000

# Run the application
CMD ["./todo-server"]
