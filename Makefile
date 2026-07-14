# Makefile for Todo Application
# Provides convenient commands for building and running the project

.PHONY: help build build-server build-client run run-server run-client test clean db-setup db-reset generate

# Configuration
SERVER_DIR := server
CLIENT_DIR := client
PROTO_DIR := proto

help: ## Show this help message
	@echo "Available targets:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
	@echo ""

# Build everything
build: build-server build-client ## Build both server and client

# Build Rust server
build-server: ## Build the Rust server
	cd $(SERVER_DIR) && cargo build

# Build Flutter client
build-client: ## Build the Flutter client
	cd $(CLIENT_DIR) && flutter pub get

# Generate code from proto files
generate: ## Generate code from protocol buffer definitions
	cd $(SERVER_DIR) && buf generate $(PROTO_DIR)/todo.proto

# Run everything
run: run-server run-client ## Run both server and client

# Run Rust server
run-server: ## Run the Rust server
	cd $(SERVER_DIR) && cargo run

# Run Flutter client
run-client: ## Run the Flutter client
	cd $(CLIENT_DIR) && flutter run

# Run database setup
db-setup: ## Set up PostgreSQL database
	chmod +x scripts/setup_database.sh && ./scripts/setup_database.sh

# Reset database (drop and recreate)
db-reset: ## Reset PostgreSQL database (drop and recreate)
	@echo "This will drop and recreate the database. Are you sure? (y/n)"
	@read answer
	@if [ "$$answer" = "y" ]; then \
		psql -d postgres -c "DROP DATABASE IF EXISTS todo_db;" && \
		psql -d postgres -c "DROP USER IF EXISTS todo_user;" && \
		$(MAKE) db-setup; \
	fi

# Clean build artifacts
clean: ## Clean build artifacts
	cd $(SERVER_DIR) && cargo clean
	cd $(CLIENT_DIR) && flutter clean
	find . -name "*.pb.dart" -delete
	find . -name "*.pb.rs" -delete

# Run tests
test: test-server test-client ## Run tests for both server and client

test-server: ## Run Rust server tests
	cd $(SERVER_DIR) && cargo test

test-client: ## Run Flutter client tests
	cd $(CLIENT_DIR) && flutter test

# Docker commands
docker-up: ## Start services with Docker Compose
	docker-compose up -d

docker-down: ## Stop services with Docker Compose
	docker-compose down

docker-logs: ## Show logs from Docker services
	docker-compose logs -f

# Run database migrations
migrate: ## Run database migrations
	@echo "Database migrations will be implemented in the next phase"

# Run code generation for both client and server
codegen: generate build-server build-client ## Run code generation and rebuild