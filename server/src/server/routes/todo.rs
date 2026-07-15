//! To-Do routes

use axum::{
    extract::Path,
    http::StatusCode,
    Json, Router,
};
use axum::middleware;
use std::sync::Arc;

use crate::{
    middleware::auth::{AuthenticatedUser, jwt_auth},
    models::todo::{
        CreateTodoRequest, TodoListResponse, TodoResponse, UpdateTodoRequest,
    },
    server::AppState,
};

/// Create router for to-do routes
pub fn todo_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_all_todos))
        .route("/", axum::routing::post(create_todo))
        .route("/:id", axum::routing::get(get_todo))
        .route("/:id", axum::routing::put(update_todo))
        .route("/:id", axum::routing::delete(delete_todo))
        .route_layer(middleware::from_fn(jwt_auth))
}

/// Get all to-do items for the current user
async fn get_all_todos(user: AuthenticatedUser) -> Result<Json<TodoListResponse>, StatusCode> {
    // TODO: Implement actual database query
    let user_id = user.user_id();
    log::debug!("Get all todos called for user: {}", user_id);
    Ok(Json(TodoListResponse {
        todos: vec![],
        total: 0,
    }))
}

/// Create a new to-do item
async fn create_todo(
    user: AuthenticatedUser,
    Json(request): Json<CreateTodoRequest>,
) -> Result<Json<TodoResponse>, StatusCode> {
    let user_id = user.user_id();
    log::debug!("Create todo for user {}: {:?}", user_id, request);
    // TODO: Implement actual database insertion with user_id
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Get a single to-do item by ID
async fn get_todo(
    user: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<Json<TodoResponse>, StatusCode> {
    let user_id = user.user_id();
    log::debug!("Get todo by ID: {} for user: {}", id, user_id);
    // TODO: Implement actual database query with user_id
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Update a to-do item
async fn update_todo(
    user: AuthenticatedUser,
    Path(id): Path<String>,
    Json(request): Json<UpdateTodoRequest>,
) -> Result<Json<TodoResponse>, StatusCode> {
    let user_id = user.user_id();
    log::debug!("Update todo {} for user {}: {:?}", id, user_id, request);
    // TODO: Implement actual database update with user_id
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Delete a to-do item
async fn delete_todo(
    user: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<Json<TodoResponse>, StatusCode> {
    let user_id = user.user_id();
    log::debug!("Delete todo: {} for user: {}", id, user_id);
    // TODO: Implement actual database deletion with user_id
    Err(StatusCode::NOT_IMPLEMENTED)
}
