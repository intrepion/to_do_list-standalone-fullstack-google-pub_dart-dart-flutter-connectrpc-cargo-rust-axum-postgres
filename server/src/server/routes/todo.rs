//! To-Do routes

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
};
use std::sync::Arc;

use crate::{
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
}

/// Get all to-do items for the current user
async fn get_all_todos(state: State<Arc<AppState>>) -> Result<Json<TodoListResponse>, StatusCode> {
    // TODO: Implement actual database query
    log::debug!("Get all todos called");
    Ok(Json(TodoListResponse {
        todos: vec![],
        total: 0,
    }))
}

/// Create a new to-do item
async fn create_todo(
    state: State<Arc<AppState>>,
    Json(request): Json<CreateTodoRequest>,
) -> Result<Json<TodoResponse>, StatusCode> {
    log::debug!("Create todo: {:?}", request);
    // TODO: Implement actual database insertion
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Get a single to-do item by ID
async fn get_todo(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<TodoResponse>, StatusCode> {
    log::debug!("Get todo by ID: {}", id);
    // TODO: Implement actual database query
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Update a to-do item
async fn update_todo(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(request): Json<UpdateTodoRequest>,
) -> Result<Json<TodoResponse>, StatusCode> {
    log::debug!("Update todo {}: {:?}", id, request);
    // TODO: Implement actual database update
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Delete a to-do item
async fn delete_todo(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<TodoResponse>, StatusCode> {
    log::debug!("Delete todo: {}", id);
    // TODO: Implement actual database deletion
    Err(StatusCode::NOT_IMPLEMENTED)
}
