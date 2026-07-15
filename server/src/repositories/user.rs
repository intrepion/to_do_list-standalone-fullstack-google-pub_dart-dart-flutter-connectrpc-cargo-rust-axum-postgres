//! User repository for database operations

use crate::models::user::{CreateUserRequest, User};
use anyhow::{Context, Result};
use bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

/// User repository for database operations
#[derive(Clone)]
pub struct UserRepository;

impl UserRepository {
    /// Create a new User repository
    pub fn new() -> Self {
        Self
    }

    /// Find a user by their Google ID
    pub async fn find_by_google_id(
        &self,
        conn: &mut PooledConnection<'_, PostgresConnectionManager<NoTls>>,
        google_id: &str,
    ) -> Result<Option<User>> {
        let row = conn
            .query_opt(
                "SELECT id, google_id, email, display_name, created_at, updated_at FROM users WHERE google_id = $1",
                &[&google_id],
            )
            .await
            .context("Failed to query user by google_id")?;

        match row {
            Some(row) => {
                let id: Uuid = row.get("id");
                let google_id: String = row.get("google_id");
                let email: String = row.get("email");
                let display_name: String = row.get("display_name");
                let created_at: String = row.get("created_at");
                let updated_at: String = row.get("updated_at");

                Ok(Some(User {
                    id,
                    google_id,
                    email,
                    display_name,
                    created_at,
                    updated_at,
                }))
            }
            None => Ok(None),
        }
    }

    /// Create a new user
    pub async fn create(
        &self,
        conn: &mut PooledConnection<'_, PostgresConnectionManager<NoTls>>,
        request: CreateUserRequest,
    ) -> Result<User> {
        let row = conn
            .query_one(
                "INSERT INTO users (google_id, email, display_name) VALUES ($1, $2, $3) RETURNING id, google_id, email, display_name, created_at, updated_at",
                &[&request.google_id, &request.email, &request.display_name],
            )
            .await
            .context("Failed to insert user")?;

        let id: Uuid = row.get("id");
        let google_id: String = row.get("google_id");
        let email: String = row.get("email");
        let display_name: String = row.get("display_name");
        let created_at: String = row.get("created_at");
        let updated_at: String = row.get("updated_at");

        Ok(User {
            id,
            google_id,
            email,
            display_name,
            created_at,
            updated_at,
        })
    }
}
