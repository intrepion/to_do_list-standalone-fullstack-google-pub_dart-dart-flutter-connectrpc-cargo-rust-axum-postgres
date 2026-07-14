# Implementation Roadmap

## Project Setup
Initialize Rust server project with Cargo.toml and basic directory structure
Add Rust dependencies: axum, connectrpc, tokio, serde, postgres, uuid, chrono, jwt
Initialize Flutter project for the Dart client
Add Flutter dependencies: google_sign_in, http, protobuf, connectrpc_dart
Add build tools: protoc, bufbuild/buf for protocol buffer generation
Set up Postgres database instance locally or via Docker
Create database and user for the todo application

## Shared API Definition
Define protocol buffer message for Todo item with fields: id, title, description, completed, created_at, updated_at
Define protocol buffer message for User with fields: id, google_id, email, name, created_at
Define protocol buffer service TodoService with rpc methods: CreateTodo, GetTodo, ListTodos, UpdateTodo, DeleteTodo
Define protocol buffer service AuthService with rpc method: Authenticate
Define protocol buffer messages for requests and responses for all rpc methods
Generate Rust server code from protocol buffers using connectrpc
Generate Dart client code from protocol buffers using connectrpc_dart

## Server Implementation - Database
Create database schema migration for users table with google_id as unique key
Create database schema migration for todos table with user_id foreign key
Set up connection pool for Postgres using sqlx or similar
Implement User repository with methods: find_by_google_id, create
Implement Todo repository with methods: create, find_by_id, list_by_user, update, delete
Add database migrations tool (sqlx-cli or similar)
Write migrations for initial schema

## Server Implementation - Authentication
Set up Google OAuth 2.0 client configuration with client_id and client_secret
Implement Google OAuth token verification endpoint
Create JWT signing and verification utilities
Implement middleware to validate JWT on protected routes
Implement AuthService Authenticate method to exchange Google token for JWT
Store user information on first login via Google OAuth

## Server Implementation - Business Logic
Implement TodoService CreateTodo method with validation
Implement TodoService GetTodo method with user ownership check
Implement TodoService ListTodos method with user filtering
Implement TodoService UpdateTodo method with user ownership check
Implement TodoService DeleteTodo method with user ownership check
Add error handling for all service methods
Implement proper HTTP status code mapping for errors

## Server Implementation - ConnectRPC
Configure connectrpc server with axum integration
Register TodoService and AuthService with the router
Set up proper content-type handling for ConnectRPC
Configure CORS for Flutter web and mobile clients
Add logging middleware for requests
Set up gRPC-web compatibility if needed

## Server Implementation - Deployment
Create configuration management for environment variables
Add Dockerfile for Rust server
Create docker-compose.yml with server and Postgres services
Set up database connection string from environment
Configure server port and host from environment

## Client Implementation - Authentication
Set up Google Sign-In configuration with OAuth client ID
Implement Google sign-in button and flow in Flutter
Handle Google OAuth token retrieval on sign-in
Implement token exchange with server Authenticate endpoint
Store JWT securely on the client (secure storage or similar)
Implement auto-login with stored token on app restart
Add logout functionality to clear stored token
Handle token expiration and refresh if needed

## Client Implementation - API Client
Generate Dart ConnectRPC client stub from protocol buffers
Create TodoService client wrapper with proper error handling
Create AuthService client wrapper with proper error handling
Add request interceptors for JWT header injection
Implement connection configuration (host, port, use_ssl)
Handle connection errors and retries gracefully

## Client Implementation - State Management
Set up state management solution (Provider, Riverpod, Bloc, etc.)
Create auth state with user info and authentication status
Create todo state with list of todos and loading states
Create shared preferences or secure storage for persistent state
Implement state updates when todos are created, updated, deleted

## Client Implementation - UI Screens
Create splash screen with app loading and auth check
Create login screen with Google sign-in button
Create home screen with todo list
Create todo detail screen to view single todo
Create add todo screen with form for title and description
Create edit todo screen with pre-filled form
Create settings screen with logout option

## Client Implementation - UI Components
Create reusable todo card widget with title, description, completed status
Create todo list widget with scrollable list of todo cards
Create floating action button for adding new todos
Create checkbox for marking todos as complete/incomplete
Create text input fields for todo title and description
Create dialog for confirming todo deletion
Create loading indicators for async operations
Create error message display for API errors

## Client Implementation - Navigation
Set up Flutter navigation with named routes
Implement navigation from login to home on successful auth
Implement navigation from home to add todo screen
Implement navigation from todo card to detail/edit screen
Implement navigation back to home after operations complete
Handle deep linking if needed

## Client Implementation - Data Flow
Connect Google sign-in to auth state and persist JWT
Connect todo list widget to todo state via state management
Connect todo operations to API client calls
Update UI after successful API responses
Show loading states during API calls
Show error messages from failed API calls
Refresh todo list after create, update, delete operations

## Testing
Write unit tests for User repository methods
Write unit tests for Todo repository methods
Write integration tests for AuthService
Write integration tests for TodoService
Write end-to-end tests for login flow
Write end-to-end tests for todo CRUD operations
Test on iOS simulator/device
Test on Android emulator/device
Test on web if applicable

## Deployment Preparation
Set up production database with proper backups
Configure production server environment variables
Set up SSL/TLS certificates for production
Configure CORS for production domains
Create build pipelines for Rust server
Create build pipelines for Flutter client (iOS, Android, Web)
Set up monitoring and logging for production
Document API endpoints and authentication flow

## Final Steps
Perform end-to-end testing of complete authentication flow
Perform end-to-end testing of complete todo CRUD flow
Verify error handling works correctly across all features
Verify UI is responsive on different screen sizes
Optimize performance for large todo lists
Final code review and cleanup
Create release builds for all platforms
