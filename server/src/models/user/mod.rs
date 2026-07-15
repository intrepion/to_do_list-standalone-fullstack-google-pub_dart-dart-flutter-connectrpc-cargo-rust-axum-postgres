//! User model

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A user entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier
    pub id: Uuid,
    /// Google ID for OAuth authentication
    pub google_id: String,
    /// User's email address
    pub email: String,
    /// User's display name
    pub display_name: String,
    /// Created at timestamp (RFC 3339)
    pub created_at: String,
    /// Updated at timestamp (RFC 3339)
    pub updated_at: String,
}

/// DTO for creating a new user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub google_id: String,
    pub email: String,
    pub display_name: String,
}
