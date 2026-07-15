//! JWT Authentication Middleware

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    body::Body,
};
use std::sync::Arc;

use crate::{
    jwt::{UserClaims, UserJwtClaims},
    server::AppState,
};

/// Authentication error types
#[derive(Debug)]
pub enum AuthError {
    /// No authorization header provided
    MissingAuthorizationHeader,
    /// Invalid authorization header format
    InvalidAuthorizationHeader,
    /// Missing Bearer token
    MissingBearerToken,
    /// Invalid JWT token
    InvalidToken(String),
    /// Token has expired
    ExpiredToken,
    /// Invalid token signature
    InvalidSignature,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::MissingAuthorizationHeader => write!(f, "Missing authorization header"),
            AuthError::InvalidAuthorizationHeader => write!(f, "Invalid authorization header format"),
            AuthError::MissingBearerToken => write!(f, "Missing Bearer token"),
            AuthError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            AuthError::ExpiredToken => write!(f, "Token has expired"),
            AuthError::InvalidSignature => write!(f, "Invalid token signature"),
        }
    }
}

impl std::error::Error for AuthError {}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match self {
            AuthError::MissingAuthorizationHeader => StatusCode::UNAUTHORIZED,
            AuthError::InvalidAuthorizationHeader => StatusCode::BAD_REQUEST,
            AuthError::MissingBearerToken => StatusCode::UNAUTHORIZED,
            AuthError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            AuthError::ExpiredToken => StatusCode::UNAUTHORIZED,
            AuthError::InvalidSignature => StatusCode::UNAUTHORIZED,
        };

        let body = match self {
            AuthError::InvalidToken(msg) => format!(r#"{{"error": "{}"}}"#, msg),
            _ => format!(r#"{{"error": "{}"}}"#, self),
        };

        (status, body).into_response()
    }
}

/// Extracted JWT claims from the request
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    /// User JWT claims
    pub claims: UserJwtClaims,
}

impl AuthenticatedUser {
    /// Get the user ID (subject) from the claims
    pub fn user_id(&self) -> &str {
        &self.claims.sub
    }

    /// Get the user email if available
    pub fn email(&self) -> Option<&String> {
        self.claims.custom.email.as_ref()
    }

    /// Get the user name if available
    pub fn name(&self) -> Option<&String> {
        self.claims.custom.name.as_ref()
    }

    /// Get the Google user ID if available
    pub fn google_id(&self) -> Option<&String> {
        self.claims.custom.google_id.as_ref()
    }
}

/// Extract authenticated user from request parts
#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the application state
        let app_state = parts
            .extensions
            .get::<Arc<AppState>>()
            .ok_or(AuthError::MissingAuthorizationHeader)?
            .clone();

        // Extract the authorization header
        let auth_header = parts
            .headers
            .get("authorization")
            .ok_or(AuthError::MissingAuthorizationHeader)?;

        let auth_str = auth_header
            .to_str()
            .map_err(|_| AuthError::InvalidAuthorizationHeader)?;

        // Parse the Bearer token
        let token = if let Some(token) = auth_str.strip_prefix("Bearer ") {
            token
        } else {
            return Err(AuthError::MissingBearerToken);
        };

        // Verify the JWT token
        match app_state.jwt_manager.verify::<UserClaims>(token) {
            Ok(claims) => Ok(AuthenticatedUser { claims }),
            Err(e) => {
                let error_msg = e.to_string();
                log::warn!("JWT verification failed: {}", error_msg);
                
                // Check for specific error types
                if error_msg.contains("expired") || error_msg.contains("Expired") {
                    Err(AuthError::ExpiredToken)
                } else if error_msg.contains("signature") || error_msg.contains("Signature") {
                    Err(AuthError::InvalidSignature)
                } else {
                    Err(AuthError::InvalidToken(error_msg))
                }
            }
        }
    }
}

/// JWT validation middleware that checks for a valid JWT token
/// 
/// This middleware extracts the JWT token from the Authorization header,
/// verifies it using the JwtManager from the application state, and either
/// allows the request to continue or returns an authentication error.
pub async fn jwt_auth(request: Request<Body>, next: Next) -> Result<Response, AuthError> {
    // Extract the application state from request extensions
    // The state is inserted by axum's with_state layer
    let app_state = request
        .extensions()
        .get::<Arc<AppState>>()
        .ok_or(AuthError::MissingAuthorizationHeader)?
        .clone();

    // Extract the authorization header
    let authorization = request
        .headers()
        .get("authorization")
        .ok_or(AuthError::MissingAuthorizationHeader)?;

    let auth_str = authorization
        .to_str()
        .map_err(|_| AuthError::InvalidAuthorizationHeader)?;

    // Parse the Bearer token
    let token = if let Some(token) = auth_str.strip_prefix("Bearer ") {
        token.to_string()
    } else {
        return Err(AuthError::MissingBearerToken);
    };

    // Verify the JWT token
    match app_state.jwt_manager.verify::<UserClaims>(&token) {
        Ok(_) => {
            // If valid, continue to the next middleware/handler
            Ok(next.run(request).await)
        }
        Err(e) => {
            let error_msg = e.to_string();
            log::warn!("JWT verification failed: {}", error_msg);
            
            if error_msg.contains("expired") || error_msg.contains("Expired") {
                Err(AuthError::ExpiredToken)
            } else if error_msg.contains("signature") || error_msg.contains("Signature") {
                Err(AuthError::InvalidSignature)
            } else {
                Err(AuthError::InvalidToken(error_msg))
            }
        }
    }
}
