# To-Do List Fullstack Implementation Roadmap

This document outlines the progressive MVPs (Minimum Viable Products) for implementing a to-do list application using:
- **Google OAuth 2.0** for authentication
- **Dart + Flutter** for the client
- **ConnectRPC** for API architecture
- **Rust + axum + connectrpc crate** for the server
- **Postgres** for storage

Each MVP builds upon the previous one and includes verification steps to manually confirm functionality.

---

## MVP 0: Project Scaffolding & Development Environment

### Objective
Set up the development environment and project structure for both client and server.

### Server (Rust) Setup
- [ ] Initialize Cargo project with `cargo init`
- [ ] Add dependencies to `Cargo.toml`:
  - `connectrpc` (latest version)
  - `axum` (web framework)
  - `tokio` (async runtime)
  - `serde` (serialization)
  - `prost` (protobuf)
  - `sqlx` or `diesel` (Postgres ORM)
  - `dotenv` (environment variables)
  - `oauth2` (Google OAuth client)
  - `jsonwebtoken` or `google-authz` (JWT handling)
- [ ] Create project structure:
  ```
  server/
  ├── src/
  │   ├── main.rs
  │   ├── config.rs
  │   ├── error.rs
  │   ├── routes.rs
  │   ├── services/
  │   └── models/
  ├── proto/
  │   └── todo.v1.proto
  ├── migrations/
  └── .env.example
  ```

### Client (Flutter) Setup
- [ ] Create Flutter project: `flutter create client`
- [ ] Add dependencies to `pubspec.yaml`:
  - `flutter_connectrpc` or `connectrpc_dart`
  - `google_sign_in` (OAuth 2.0)
  - `http` or `dio`
  - `provider` or `riverpod` (state management)
  - `shared_preferences` (local storage for tokens)
  - `intl` (optional, for localization)
- [ ] Create project structure:
  ```
  client/
  ├── lib/
  │   ├── main.dart
  │   ├── app.dart
  │   ├── config/
  │   ├── services/
  │   ├── models/
  │   ├── screens/
  │   ├── widgets/
  │   └── providers/
  └── pubspec.yaml
  ```

### Shared Protobuf Schema
- [ ] Define initial `.proto` file in `server/proto/todo.v1.proto`
- [ ] Include basic message types for future use

### Infrastructure
- [ ] Set up local Postgres instance (Docker recommended)
- [ ] Create database and user with appropriate permissions
- [ ] Configure Google OAuth 2.0 credentials in Google Cloud Console
- [ ] Set up `.env.example` files for both server and client

### Verification
✅ Server compiles with `cargo check`
✅ Client compiles with `flutter analyze`
✅ Postgres is running and accessible
✅ Google OAuth credentials are configured

---

## MVP 1: Foundation - Server & Database Connection

### Objective
Establish the server foundation with database connectivity and basic health endpoints.

### Server Implementation
- [ ] Create `config.rs` with:
  - Database connection string from environment
  - Server port configuration
  - CORS settings
- [ ] Create `error.rs` with custom error types
- [ ] Implement database connection pool using `sqlx`
- [ ] Create initial migrations:
  ```sql
  -- users table (for future use)
  CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    google_id VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    display_name VARCHAR(255),
    avatar_url VARCHAR(512),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
  );
  ```
- [ ] Create `main.rs` with:
  - Axum router setup
  - Health check endpoint (`GET /health`)
  - Database connection test endpoint
  - CORS middleware
  - Error handling middleware
- [ ] Add ConnectRPC service definition (empty for now)

### Verification
✅ Server starts successfully with `cargo run`
✅ `GET /health` returns 200 OK
✅ Database connection endpoint returns success
✅ Database tables are created via migrations

---

## MVP 2: Authentication - Google OAuth 2.0 Flow

### Objective
Implement Google OAuth 2.0 authentication flow for user login.

### Server Implementation
- [ ] Add Google OAuth configuration to `config.rs`:
  - Client ID
  - Client secret
  - Redirect URIs
- [ ] Create `auth.rs` service with:
  - OAuth 2.0 client setup
  - Token exchange endpoint (`POST /auth/token`)
  - Token validation function
  - User creation/upsert on first login
- [ ] Create JWT utility for session management
- [ ] Add `users` table migration (if not already created)
- [ ] Implement authentication middleware for Axum

### Client Implementation
- [ ] Configure `google_sign_in` package with:
  - iOS and Android OAuth client IDs
  - Requested scopes (email, profile)
- [ ] Create `auth_service.dart` with:
  - `signInWithGoogle()` method
  - `signOut()` method
  - Token storage in `shared_preferences`
  - Token refresh logic
- [ ] Create `auth_provider.dart` (using Riverpod):
  - Auth state management (Authenticated, Unauthenticated, Loading)
  - User data storage
- [ ] Create basic auth screens:
  - Login screen with Google Sign-In button
  - Loading state
  - Error state

### Shared Schema
- [ ] Update `todo.v1.proto` with auth-related messages:
  ```protobuf
  message AuthRequest {
    string token = 1;
  }
  message AuthResponse {
    string jwt = 1;
    User user = 2;
  }
  message User {
    string id = 1;
    string google_id = 2;
    string email = 3;
    string display_name = 4;
    string avatar_url = 5;
  }
  ```

### ConnectRPC Service
- [ ] Define `AuthService` in proto:
  ```protobuf
  service AuthService {
    rpc Login (AuthRequest) returns (AuthResponse);
    rpc Validate (JwtRequest) returns (ValidateResponse);
  }
  ```
- [ ] Implement server-side AuthService
- [ ] Generate Dart client stubs

### Verification
✅ Google Sign-In button works in Flutter app
✅ Token exchange completes successfully
✅ User is redirected/navigated on successful login
✅ Token is stored and persists across app restarts
✅ Invalid tokens are rejected
✅ Sign out clears tokens

---

## MVP 3: Core API - ConnectRPC Service with Authentication

### Objective
Establish working ConnectRPC communication between client and server with auth interceptors.

### Server Implementation
- [ ] Set up ConnectRPC router integration with Axum
- [ ] Create interceptors:
  - Authentication interceptor (validates JWT)
  - Error handling interceptor
  - Logging interceptor
- [ ] Create base service structure
- [ ] Implement gRPC health check endpoint

### Client Implementation
- [ ] Set up ConnectRPC client:
  - Create channel configuration
  - Add base URL from environment
  - Add error handling
- [ ] Create interceptors:
  - Auth interceptor (adds JWT to requests)
  - Error interceptor (handles auth failures)
- [ ] Create base repository class for API calls

### Shared Schema
- [ ] Define common messages in `todo.v1.proto`:
  ```protobuf
  message Empty {}
  message HealthCheckResponse {
    string status = 1;
  }
  message Error {
    int32 code = 1;
    string message = 2;
  }
  ```
- [ ] Define service options:
  ```protobuf
  service HealthService {
    rpc Check (Empty) returns (HealthCheckResponse);
  }
  ```

### Verification
✅ Client can connect to server via ConnectRPC
✅ Health check endpoint responds successfully
✅ Authentication interceptor adds JWT to requests
✅ Unauthenticated requests are rejected with 401
✅ Error responses are properly parsed on client

---

## MVP 4: To-Do List Core - CRUD Operations

### Objective
Implement basic to-do list functionality: Create, Read, Update, Delete.

### Server Implementation
- [ ] Create `todos` table migration:
  ```sql
  CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(512) NOT NULL,
    description TEXT,
    completed BOOLEAN DEFAULT false,
    priority INTEGER DEFAULT 0, -- 0 = none, 1 = low, 2 = medium, 3 = high
    due_date TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    INDEX idx_todos_user_id (user_id)
  );
  ```
- [ ] Create `models/todo.rs` with:
  - Todo struct
  - Database row mapping
  - CRUD operations
- [ ] Create `services/todo_service.rs` with business logic
- [ ] Add ConnectRPC endpoints:
  - `ListTodos` - Get all todos for user
  - `CreateTodo` - Create a new todo
  - `GetTodo` - Get a specific todo
  - `UpdateTodo` - Update a todo
  - `DeleteTodo` - Delete a todo

### Client Implementation
- [ ] Create `models/todo.dart` with:
  - Todo class
  - fromJson/toJson methods
  - Equality and copyWith methods
- [ ] Create `repositories/todo_repository.dart` with:
  - All CRUD methods
  - Connection to ConnectRPC service
- [ ] Create `providers/todo_provider.dart` (Riverpod) with:
  - Todo list state
  - Loading states
  - Error handling
- [ ] Create basic screens:
  - Todo list screen (display all todos)
  - Add todo screen (form with title, description)
  - Todo detail screen (view and edit)
- [ ] Create basic widgets:
  - Todo card
  - Todo form
  - Delete confirmation dialog

### Shared Schema
- [ ] Update `todo.v1.proto` with todo messages:
  ```protobuf
  message Todo {
    int64 id = 1;
    string title = 2;
    string description = 3;
    bool completed = 4;
    int32 priority = 5;
    google.protobuf.Timestamp due_date = 6;
    google.protobuf.Timestamp created_at = 7;
    google.protobuf.Timestamp updated_at = 8;
  }
  
  message ListTodosRequest {
    // Empty for now, may add filters later
  }
  
  message ListTodosResponse {
    repeated Todo todos = 1;
  }
  
  message CreateTodoRequest {
    string title = 1;
    string description = 2;
    int32 priority = 3;
    google.protobuf.Timestamp due_date = 4;
  }
  
  message CreateTodoResponse {
    Todo todo = 1;
  }
  
  message GetTodoRequest {
    int64 id = 1;
  }
  
  message UpdateTodoRequest {
    int64 id = 1;
    string title = 2;
    string description = 3;
    bool completed = 4;
    int32 priority = 5;
    google.protobuf.Timestamp due_date = 6;
  }
  
  message DeleteTodoRequest {
    int64 id = 1;
  }
  ```
- [ ] Define TodoService:
  ```protobuf
  service TodoService {
    rpc ListTodos (ListTodosRequest) returns (ListTodosResponse);
    rpc CreateTodo (CreateTodoRequest) returns (CreateTodoResponse);
    rpc GetTodo (GetTodoRequest) returns (Todo);
    rpc UpdateTodo (UpdateTodoRequest) returns (Todo);
    rpc DeleteTodo (DeleteTodoRequest) returns (Empty);
  }
  ```

### Verification
✅ Todos can be created and appear in the list
✅ Todos can be marked as complete/incomplete
✅ Todos can be edited (title, description)
✅ Todos can be deleted
✅ All todos are scoped to the authenticated user
✅ User A cannot see User B's todos
✅ UI updates reactively when todos change

---

## MVP 5: Enhanced Features - Filtering, Sorting, Priority

### Objective
Add filtering, sorting, and priority handling to the to-do list.

### Server Implementation
- [ ] Update `ListTodos` endpoint to accept:
  - Filter by completion status
  - Filter by priority
  - Sort by due date, created date, priority
  - Pagination (limit, offset)
- [ ] Add database indexes for filtering/sorting
- [ ] Implement query builder for complex filtering

### Client Implementation
- [ ] Add filter controls to todo list screen:
  - Toggle: All/Active/Completed
  - Priority filter dropdown
  - Sort order dropdown
- [ ] Add priority selection to todo form
- [ ] Add priority indicators (colors/icons) to todo cards
- [ ] Implement infinite scroll or pagination controls

### Shared Schema
- [ ] Update `ListTodosRequest`:
  ```protobuf
  message ListTodosRequest {
    bool completed = 1; // optional, filters by completion
    int32 priority = 2; // optional, filters by priority
    string sort_by = 3; // "created", "updated", "due_date", "priority"
    bool sort_descending = 4;
    int32 limit = 5;
    int32 offset = 6;
  }
  ```
- [ ] Update `ListTodosResponse`:
  ```protobuf
  message ListTodosResponse {
    repeated Todo todos = 1;
    int32 total_count = 2;
  }
  ```

### Verification
✅ Todos can be filtered by completion status
✅ Todos can be filtered by priority
✅ Todos can be sorted by different criteria
✅ Priority is visually indicated in the UI
✅ Priority can be set when creating/editing todos
✅ Pagination works correctly (if implemented)

---

## MVP 6: Due Dates & Reminders

### Objective
Add due date functionality with overdue indicators and basic reminder notifications.

### Server Implementation
- [ ] Add due date validation (past dates allowed)
- [ ] Add query for overdue todos
- [ ] Create endpoint for upcoming todos (due within N days)

### Client Implementation
- [ ] Add date picker to todo form
- [ ] Display due date in todo card
- [ ] Highlight overdue todos (red indicator)
- [ ] Highlight due-today todos (orange indicator)
- [ ] Add due date to todo detail screen
- [ ] Create upcoming todos section/widget

### Verification
✅ Due dates can be set on todos
✅ Overdue todos are visually distinct
✅ Today's due todos are visually distinct
✅ Upcoming todos can be viewed separately

---

## MVP 7: User Profile & Settings

### Objective
Add user profile management and basic settings.

### Server Implementation
- [ ] Add `users` endpoint for profile retrieval
- [ ] Add profile update endpoint (display name, avatar)
- [ ] Add password/email change endpoints (if using password auth)

### Client Implementation
- [ ] Create profile screen with:
  - User avatar
  - Display name
  - Email
  - Google account info
- [ ] Add settings screen with:
  - Theme toggle (light/dark)
  - Notification settings
  - Sign out button
- [ ] Create drawer or sidebar navigation

### Shared Schema
- [ ] Add profile-related messages:
  ```protobuf
  message GetProfileRequest {}
  message UpdateProfileRequest {
    string display_name = 1;
    string avatar_url = 2;
  }
  ```
- [ ] Add ProfileService:
  ```protobuf
  service ProfileService {
    rpc GetProfile (GetProfileRequest) returns (User);
    rpc UpdateProfile (UpdateProfileRequest) returns (User);
  }
  ```

### Verification
✅ User profile is displayed correctly
✅ Profile can be updated (display name)
✅ Settings screen is accessible
✅ Theme can be toggled
✅ Sign out works from settings

---

## MVP 8: Error Handling & Edge Cases

### Objective
Handle edge cases, improve error handling, and add robust validation.

### Server Implementation
- [ ] Add comprehensive validation for all inputs
- [ ] Create proper error messages for:
  - Invalid tokens
  - Expired tokens
  - Missing permissions
  - Database errors
  - Rate limiting
- [ ] Add rate limiting middleware
- [ ] Add request logging
- [ ] Implement proper HTTP status codes in ConnectRPC responses

### Client Implementation
- [ ] Add error handling for:
  - Network errors
  - Authentication failures
  - Form validation errors
- [ ] Create error boundary components
- [ ] Add retry logic for failed requests
- [ ] Create user-friendly error messages
- [ ] Add loading skeletons for async operations

### Verification
✅ Invalid inputs show clear validation errors
✅ Network errors are handled gracefully
✅ Auth errors redirect to login
✅ Rate limiting is enforced
✅ Errors are logged on server
✅ UI shows appropriate loading states
✅ Failed operations can be retried

---

## MVP 9: Testing & Production Readiness

### Objective
Add tests, documentation, and prepare for production deployment.

### Server Testing
- [ ] Add unit tests for:
  - Auth service
  - Todo service
  - Validation logic
- [ ] Add integration tests:
  - API endpoints
  - Database operations
  - Auth flow
- [ ] Add error handling tests

### Client Testing
- [ ] Add widget tests for critical components
- [ ] Add integration tests for auth flow
- [ ] Add mock API tests for todo operations

### Documentation
- [ ] Add API documentation (ConnectRPC generates some automatically)
- [ ] Add OpenAPI/Swagger documentation (optional)
- [ ] Create setup guides for development
- [ ] Add deployment documentation

### Production Preparation
- [ ] Add Docker configuration for server
- [ ] Add Docker Compose for local development (server + postgres)
- [ ] Configure production environment variables
- [ ] Set up CI/CD pipeline
- [ ] Add health checks and monitoring endpoints
- [ ] Configure proper CORS for production

### Security
- [ ] Review OAuth 2.0 token handling
- [ ] Ensure JWT secrets are properly secured
- [ ] Add input sanitization
- [ ] Review SQL injection protection
- [ ] Add HTTPS configuration

### Performance
- [ ] Add database query optimization
- [ ] Implement caching where appropriate
- [ ] Add query analysis for slow queries

### Verification
✅ All tests pass
✅ Server runs in Docker container
✅ Local development works with Docker Compose
✅ API documentation is available
✅ Security review completed
✅ Performance is acceptable under load

---

## MVP 10: Polish & User Experience

### Objective
Final polish for production-ready user experience.

### Client Enhancements
- [ ] Add animations for:
  - Todo creation
  - Todo completion
  - Todo deletion
  - Screen transitions
- [ ] Add empty state screens
- [ ] Add pull-to-refresh on todo list
- [ ] Add swipe to delete gesture
- [ ] Add search functionality
- [ ] Add keyboard shortcuts (for web)
- [ ] Add proper form validation with feedback

### Server Enhancements
- [ ] Add request/response logging
- [ ] Add metrics collection (optional)
- [ ] Add structured error responses

### Verification
✅ UI feels polished and responsive
✅ All animations are smooth
✅ Empty states are helpful
✅ Pull-to-refresh works
✅ Swipe to delete works
✅ Search filters todos correctly
✅ Form validation provides clear feedback

---

## Deployment Checklist

### Before First Deployment
- [ ] All MVPs 0-9 are complete
- [ ] Security review completed
- [ ] Performance testing completed
- [ ] Backup strategy in place
- [ ] Monitoring configured
- [ ] Domain and SSL configured

### Deployment Steps
1. Set up production Postgres database
2. Run migrations on production database
3. Configure production environment variables
4. Build and deploy server
5. Configure client with production server URL
6. Build and deploy client (App Store, Play Store, Web)
7. Verify all functionality in production

---

## Technology Stack Summary

| Component | Technology |
|-----------|------------|
| Authentication | Google OAuth 2.0 |
| Client Framework | Flutter (Dart) |
| Client State Management | Riverpod (recommended) or Provider |
| Client Networking | connectrpc_dart / flutter_connectrpc |
| Client OAuth | google_sign_in |
| Client Storage | shared_preferences |
| API Architecture | ConnectRPC (Protocol Buffers + HTTP/2) |
| Server Framework | axum |
| Server Language | Rust |
| Server RPC | connectrpc crate |
| Server Database | Postgres (via sqlx or diesel) |
| Server Auth | oauth2 crate + JWT |
| Server Config | dotenv |
| Database | Postgres |

---

## Estimated Complexity

| MVP | Complexity | Estimated Time |
|-----|------------|----------------|
| MVP 0 | Low | 1-2 days |
| MVP 1 | Low-Medium | 1-3 days |
| MVP 2 | Medium-High | 3-5 days |
| MVP 3 | Medium | 2-3 days |
| MVP 4 | Medium-High | 3-5 days |
| MVP 5 | Medium | 2-3 days |
| MVP 6 | Low-Medium | 1-2 days |
| MVP 7 | Medium | 2-3 days |
| MVP 8 | Medium | 2-4 days |
| MVP 9 | Medium-High | 3-5 days |
| MVP 10 | Low-Medium | 2-3 days |

*Note: Time estimates are approximate and depend on familiarity with the technologies.*

---

## Getting Started

1. Start with **MVP 0** - Set up your development environment
2. Progress through each MVP in order, verifying each before moving to the next
3. Use the verification checklists to manually test each feature
4. Once all MVPs are complete, proceed to deployment

---

## Resources

- [ConnectRPC Documentation](https://connectrpc.com/)
- [connectrpc Rust Crate](https://crates.io/crates/connectrpc)
- [Axum Framework](https://github.com/tokio-rs/axum)
- [google_sign_in Flutter Package](https://pub.dev/packages/google_sign_in)
- [connectrpc_dart Package](https://pub.dev/packages/connectrpc_dart)
- [sqlx for Rust](https://github.com/launchbadge/sqlx)
- [Google OAuth 2.0](https://developers.google.com/identity/protocols/oauth2)