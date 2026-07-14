#!/bin/bash

# Database Setup Script for Todo Application
# This script sets up PostgreSQL database and user for the todo application

set -e

echo "Setting up PostgreSQL database for Todo Application..."

# Check if psql is available
if ! command -v psql &> /dev/null; then
    echo "Error: psql command not found. Please install PostgreSQL first."
    exit 1
fi

# Database configuration
DB_NAME="todo_db"
DB_USER="todo_user"
DB_PASSWORD="todo_password"

# Check if database already exists
echo "Checking if database '$DB_NAME' exists..."
if psql -lqt | cut -d \| -f 1 | grep -qw "$DB_NAME"; then
    echo "Database '$DB_NAME' already exists."
else
    echo "Creating database '$DB_NAME'..."
    createdb "$DB_NAME"
fi

# Check if user already exists
echo "Checking if user '$DB_USER' exists..."
if psql -tAc "SELECT 1 FROM pg_roles WHERE rolname='$DB_USER'" | grep -q 1; then
    echo "User '$DB_USER' already exists."
else
    echo "Creating user '$DB_USER'..."
    createuser "$DB_USER" -P
    # Set password non-interactively
    psql -c "ALTER USER \"$DB_USER\" WITH PASSWORD '$DB_PASSWORD'"
fi

# Grant privileges to user on database
echo "Granting privileges to '$DB_USER' on '$DB_NAME'..."
psql -c "GRANT ALL PRIVILEGES ON DATABASE \"$DB_NAME\" TO \"$DB_USER\""

# Create tables
echo "Creating database schema..."
psql "$DB_NAME" << EOF
-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    google_id VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Todos table
CREATE TABLE IF NOT EXISTS todos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    completed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_todos_user_id ON todos(user_id);
CREATE INDEX IF NOT EXISTS idx_users_google_id ON users(google_id);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Grant table privileges to user
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO "$DB_USER";
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO "$DB_USER";
EOF

echo "Database setup completed successfully!"
echo "Connection string: postgres://$DB_USER:$DB_PASSWORD@localhost:5432/$DB_NAME"

# Test the connection
echo "Testing database connection..."
PGPASSWORD="$DB_PASSWORD" psql -U "$DB_USER" -d "$DB_NAME" -c "SELECT 'Database connection successful' as message;"

echo "Setup completed!"
