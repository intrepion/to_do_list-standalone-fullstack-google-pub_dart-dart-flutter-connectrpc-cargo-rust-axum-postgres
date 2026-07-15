# Makefile for protocol buffer code generation and database migrations

.PHONY: gen gen-rust gen-dart clean db-migrate db-setup db-reset db-info install-sqlx

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

# Database migration commands
# Note: Requires DATABASE_URL environment variable to be set
# Example: DATABASE_URL=postgres://user:password@localhost:5432/to_do_list make db-migrate

# Check if sqlx is installed, if not install it
db-check-sqlx:
	@if ! command -v sqlx >/dev/null 2>&1; then \
		 echo "sqlx-cli not found. Installing..." && \
		 cargo install sqlx-cli --features postgres --locked; \
	fi

db-migrate: db-check-sqlx
	@echo "Running database migrations..."
	@cd server && DATABASE_URL=$(DATABASE_URL) sqlx migrate run

db-setup: db-check-sqlx
	@echo "Setting up database (creates database and runs migrations)..."
	@cd server && DATABASE_URL=$(DATABASE_URL) sqlx database setup

db-reset: db-check-sqlx
	@echo "Resetting database (drops and recreates)..."
	@cd server && DATABASE_URL=$(DATABASE_URL) sqlx database reset

db-info: db-check-sqlx
	@echo "Database migration info..."
	@cd server && DATABASE_URL=$(DATABASE_URL) sqlx migrate info

# Install sqlx-cli globally
install-sqlx:
	cargo install sqlx-cli --features postgres --locked
