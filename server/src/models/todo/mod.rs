//! To-Do item model

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A to-do item entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    /// Unique identifier
    pub id: Uuid,
    /// User ID who owns this to-do item
    pub user_id: Uuid,
    /// Title of the to-do item
    pub title: String,
    /// Description (optional)
    pub description: Option<String>,
    /// Whether the item is completed
    pub completed: bool,
    /// Priority level (1-5, where 5 is highest)
    pub priority: i32,
    /// Due date (optional, as RFC 3339 timestamp)
    pub due_date: Option<String>,
    /// Created at timestamp (RFC 3339)
    pub created_at: String,
    /// Updated at timestamp (RFC 3339)
    pub updated_at: String,
}

/// DTO for creating a new to-do item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<i32>,
    pub due_date: Option<String>,
}

/// DTO for updating a to-do item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
    pub priority: Option<i32>,
    pub due_date: Option<String>,
}

/// Response wrapper for to-do items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoResponse {
    pub todo: Todo,
}

/// List response for multiple to-do items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoListResponse {
    pub todos: Vec<Todo>,
    pub total: usize,
}
