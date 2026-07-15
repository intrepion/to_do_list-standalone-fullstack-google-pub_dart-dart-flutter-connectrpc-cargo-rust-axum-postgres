//! Todo repository for database operations

use crate::models::todo::{CreateTodoRequest, Todo, UpdateTodoRequest};
use anyhow::{Context, Result};
use bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

/// Todo repository for database operations
#[derive(Clone)]
pub struct TodoRepository;

impl TodoRepository {
    /// Create a new Todo repository
    pub fn new() -> Self {
        Self
    }

    /// Create a new todo item
    pub async fn create(
        &self,
        conn: &mut PooledConnection<'_, PostgresConnectionManager<NoTls>>,
        user_id: Uuid,
        request: CreateTodoRequest,
    ) -> Result<Todo> {
        let row = conn
            .query_one(
                "INSERT INTO todos (user_id, title, description, completed, priority, due_date) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, user_id, title, description, completed, priority, due_date, created_at, updated_at",
                &[&
                    user_id,
                    &request.title,
                    &request.description,
                    &false, // completed defaults to false
                    &(request.priority.unwrap_or(1)),
                    &request.due_date,
                ],
            )
            .await
            .context("Failed to insert todo")?;

        let id: Uuid = row.get("id");
        let user_id: Uuid = row.get("user_id");
        let title: String = row.get("title");
        let description: Option<String> = row.get("description");
        let completed: bool = row.get("completed");
        let priority: i32 = row.get("priority");
        let due_date: Option<String> = row.get("due_date");
        let created_at: String = row.get("created_at");
        let updated_at: String = row.get("updated_at");

        Ok(Todo {
            id,
            user_id,
            title,
            description,
            completed,
            priority,
            due_date,
            created_at,
            updated_at,
        })
    }

    /// Find a todo by its ID
    pub async fn find_by_id(
        &self,
        conn: &mut PooledConnection<'_, PostgresConnectionManager<NoTls>>,
        id: Uuid,
    ) -> Result<Option<Todo>> {
        let row = conn
            .query_opt(
                "SELECT id, user_id, title, description, completed, priority, due_date, created_at, updated_at FROM todos WHERE id = $1",
                &[&id],
            )
            .await
            .context("Failed to query todo by id")?;

        match row {
            Some(row) => {
                let id: Uuid = row.get("id");
                let user_id: Uuid = row.get("user_id");
                let title: String = row.get("title");
                let description: Option<String> = row.get("description");
                let completed: bool = row.get("completed");
                let priority: i32 = row.get("priority");
                let due_date: Option<String> = row.get("due_date");
                let created_at: String = row.get("created_at");
                let updated_at: String = row.get("updated_at");

                Ok(Some(Todo {
                    id,
                    user_id,
                    title,
                    description,
                    completed,
                    priority,
                    due_date,
                    created_at,
                    updated_at,
                }))
            }
            None => Ok(None),
        }
    }

    /// List all todos for a specific user
    pub async fn list_by_user(
        &self,
        conn: &mut PooledConnection<'_, PostgresConnectionManager<NoTls>>,
        user_id: Uuid,
    ) -> Result<Vec<Todo>> {
        let rows = conn
            .query(
                "SELECT id, user_id, title, description, completed, priority, due_date, created_at, updated_at FROM todos WHERE user_id = $1 ORDER BY created_at DESC",
                &[&user_id],
            )
            .await
            .context("Failed to query todos by user_id")?;

        let mut todos = Vec::new();
        for row in rows {
            let id: Uuid = row.get("id");
            let user_id: Uuid = row.get("user_id");
            let title: String = row.get("title");
            let description: Option<String> = row.get("description");
            let completed: bool = row.get("completed");
            let priority: i32 = row.get("priority");
            let due_date: Option<String> = row.get("due_date");
            let created_at: String = row.get("created_at");
            let updated_at: String = row.get("updated_at");

            todos.push(Todo {
                id,
                user_id,
                title,
                description,
                completed,
                priority,
                due_date,
                created_at,
                updated_at,
            });
        }

        Ok(todos)
    }

    /// Update a todo item
    pub async fn update(
        &self,
        conn: &mut PooledConnection<'_, PostgresConnectionManager<NoTls>>,
        id: Uuid,
        request: UpdateTodoRequest,
    ) -> Result<Option<Todo>> {
        // Build the update query dynamically based on which fields are present
        let mut query = "UPDATE todos SET".to_string();
        let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        let mut param_index = 1;

        if let Some(title) = &request.title {
            query.push_str(&format!(" title = ${}", param_index));
            params.push(title);
            param_index += 1;
        }

        if let Some(description) = &request.description {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" description = ${}", param_index));
            params.push(description);
            param_index += 1;
        }

        if let Some(completed) = &request.completed {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" completed = ${}", param_index));
            params.push(completed);
            param_index += 1;
        }

        if let Some(priority) = &request.priority {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" priority = ${}", param_index));
            params.push(priority);
            param_index += 1;
        }

        if let Some(due_date) = &request.due_date {
            if param_index > 1 {
                query.push_str(",");
            }
            query.push_str(&format!(" due_date = ${}", param_index));
            params.push(due_date);
            param_index += 1;
        }

        // If no fields to update, return the existing todo
        if param_index == 1 {
            return self.find_by_id(conn, id).await;
        }

        query.push_str(&format!(
            " WHERE id = ${} RETURNING id, user_id, title, description, completed, priority, due_date, created_at, updated_at",
            param_index
        ));
        params.push(&id);

        let row = conn
            .query_opt(&query, &params)
            .await
            .context("Failed to update todo")?;

        match row {
            Some(row) => {
                let id: Uuid = row.get("id");
                let user_id: Uuid = row.get("user_id");
                let title: String = row.get("title");
                let description: Option<String> = row.get("description");
                let completed: bool = row.get("completed");
                let priority: i32 = row.get("priority");
                let due_date: Option<String> = row.get("due_date");
                let created_at: String = row.get("created_at");
                let updated_at: String = row.get("updated_at");

                Ok(Some(Todo {
                    id,
                    user_id,
                    title,
                    description,
                    completed,
                    priority,
                    due_date,
                    created_at,
                    updated_at,
                }))
            }
            None => Ok(None),
        }
    }

    /// Delete a todo item
    pub async fn delete(
        &self,
        conn: &mut PooledConnection<'_, PostgresConnectionManager<NoTls>>,
        id: Uuid,
    ) -> Result<bool> {
        let rows_affected = conn
            .execute("DELETE FROM todos WHERE id = $1", &[&id])
            .await
            .context("Failed to delete todo")?;

        Ok(rows_affected > 0)
    }
}