// Route Configuration
// Sets up all API routes for the application

use axum::Router;

pub fn create_router() -> Router {
    // This will be populated with ConnectRPC routes and other endpoints
    Router::new()
        // TODO: Add ConnectRPC routes for TodoService and AuthService
        // TODO: Add health check endpoint
}
