//! Authentication service for Google OAuth token verification

use anyhow::{anyhow, Context, Result};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use connectrpc::{RequestContext, ServiceRequest, ServiceResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_postgres::NoTls;

use crate::gen::auth::v1::{AuthenticateRequest, AuthenticateResponse, OwnedAuthenticateResponseView};
use crate::jwt::{UserClaims, JwtManager};
use crate::gen::auth::v1::AuthService as ConnectAuthService;
use crate::models::user::CreateUserRequest;
use crate::repositories::user::UserRepository;

/// Google token verification response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleTokenInfo {
    /// The issuer (should be "https://accounts.google.com" or "accounts.google.com")
    pub iss: String,
    /// The subject (user's unique Google ID)
    pub sub: String,
    /// The audience (your client ID)
    pub aud: String,
    /// Email address
    pub email: String,
    /// Email verified status
    pub email_verified: bool,
    /// User's name
    pub name: Option<String>,
    /// User's picture URL
    pub picture: Option<String>,
    /// Locale
    pub locale: Option<String>,
    /// Issued at timestamp
    pub iat: Option<u64>,
    /// Expiration timestamp
    pub exp: Option<u64>,
}

/// Request body for token verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenVerificationRequest {
    /// The ID token to verify
    pub id_token: String,
}

/// Response for token verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenVerificationResponse {
    /// Whether the token is valid
    pub valid: bool,
    /// The Google user ID
    pub google_id: Option<String>,
    /// The user's email
    pub email: Option<String>,
    /// The user's display name
    pub display_name: Option<String>,
    /// Error message if verification failed
    pub error: Option<String>,
}

/// Authentication service
#[derive(Clone)]
pub struct AuthService {
    client: Client,
    client_id: String,
    jwt_manager: JwtManager,
    db_pool: Pool<PostgresConnectionManager<NoTls>>,
    user_repository: UserRepository,
}

impl AuthService {
    /// Create a new authentication service with JWT manager and database pool
    pub fn new(
        client_id: String,
        jwt_manager: JwtManager,
        db_pool: Pool<PostgresConnectionManager<NoTls>>,
    ) -> Self {
        Self {
            client: Client::new(),
            client_id,
            jwt_manager,
            db_pool,
            user_repository: UserRepository::new(),
        }
    }

    /// Create a new authentication service without JWT manager (for backward compatibility)
    /// This is useful for token verification without JWT signing
    /// Note: This creates a dummy pool that will fail on actual use, but is fine for
    /// the verify_token endpoint which doesn't need DB access
    pub async fn new_for_verification(client_id: String) -> Self {
        // Create a dummy JWT manager - this service won't be used for JWT signing
        let jwt_manager = JwtManager::new("dummy-secret".to_string(), chrono::Duration::hours(1));
        // Create a dummy pool - this service won't be used for database operations
        let mgr = PostgresConnectionManager::new_from_stringlike("", NoTls).unwrap();
        let db_pool = bb8::Pool::builder().build(mgr).await.unwrap();
        Self {
            client: Client::new(),
            client_id,
            jwt_manager,
            db_pool,
            user_repository: UserRepository::new(),
        }
    }

    /// Verify a Google ID token
    /// 
    /// This method verifies the ID token by calling Google's token info endpoint.
    /// The token is sent in the Authorization header as a Bearer token.
    pub async fn verify_token(&self, id_token: &str) -> Result<GoogleTokenInfo> {
        // First, decode the JWT header to check the token structure
        let parts: Vec<&str> = id_token.split('.').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid JWT format"));
        }

        // Build the Google token info URL
        let url = format!("https://www.googleapis.com/oauth2/v3/tokeninfo?id_token={}", id_token);

        // Make the request to Google's token info endpoint
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to call Google token info endpoint")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "Google token verification failed: {} {}",
                status,
                error_text
            ));
        }

        // Parse the response
        let token_info: GoogleTokenInfo = response
            .json()
            .await
            .context("Failed to parse Google token info response")?;

        // Validate the token
        self.validate_token_info(&token_info)?;

        Ok(token_info)
    }

    /// Validate the token info response
    fn validate_token_info(&self, token_info: &GoogleTokenInfo) -> Result<()> {
        // Check if the audience matches our client ID
        // Google returns aud as a string, but it might be an array in some cases
        // For simplicity, we'll check if our client_id is in the aud field
        if token_info.aud != self.client_id {
            // Try to handle case where aud might be a comma-separated list
            // This is a workaround for potential array formatting
            return Err(anyhow!(
                "Invalid audience: expected {}, got {}",
                self.client_id,
                token_info.aud
            ));
        }

        // Check if the issuer is valid
        let valid_issuers = [
            "https://accounts.google.com",
            "accounts.google.com",
        ];
        if !valid_issuers.contains(&token_info.iss.as_str()) {
            return Err(anyhow!(
                "Invalid issuer: expected one of {:?}, got {}",
                valid_issuers,
                token_info.iss
            ));
        }

        // Check if token is expired
        if let Some(exp) = token_info.exp {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| anyhow!("System time error: {}", e))?
                .as_secs();

            if now > exp {
                return Err(anyhow!("Token has expired"));
            }
        }

        // Check if email is verified (optional, depending on your requirements)
        if !token_info.email_verified {
            return Err(anyhow!("Email not verified"));
        }

        Ok(())
    }

    /// Verify token and return a standardized response
    pub async fn verify_and_response(&self, id_token: &str) -> TokenVerificationResponse {
        match self.verify_token(id_token).await {
            Ok(token_info) => TokenVerificationResponse {
                valid: true,
                google_id: Some(token_info.sub),
                email: Some(token_info.email),
                display_name: token_info.name,
                error: None,
            },
            Err(e) => TokenVerificationResponse {
                valid: false,
                google_id: None,
                email: None,
                display_name: None,
                error: Some(e.to_string()),
            },
        }
    }

    /// Check if user exists in database, create if not
    async fn find_or_create_user(&self, token_info: &GoogleTokenInfo) -> Result<String> {
        // Get a connection from the pool
        let mut conn = self
            .db_pool
            .get()
            .await
            .context("Failed to get database connection")?;

        // Try to find existing user by Google ID
        match self
            .user_repository
            .find_by_google_id(&mut conn, &token_info.sub)
            .await?
        {
            Some(user) => {
                log::debug!(
                    "Found existing user in database: {} ({})",
                    user.email, user.google_id
                );
                Ok(user.id.to_string())
            }
            None => {
                // User doesn't exist, create a new one
                log::info!(
                    "Creating new user in database: {} ({})",
                    token_info.email, token_info.sub
                );
                let display_name = token_info.name.clone().unwrap_or_else(|| "".to_string());
                let create_request = CreateUserRequest {
                    google_id: token_info.sub.clone(),
                    email: token_info.email.clone(),
                    display_name,
                };
                let user = self
                    .user_repository
                    .create(&mut conn, create_request)
                    .await
                    .context("Failed to create user")?;
                Ok(user.id.to_string())
            }
        }
    }
}

/// Implement the ConnectRPC AuthService trait for our AuthService
#[allow(refining_impl_trait)]
impl ConnectAuthService for AuthService {
    /// Handle the Authenticate RPC.
    /// 
    /// This method exchanges a Google OAuth token for a JWT token.
    /// It verifies the Google token, stores user info on first login,
    /// then creates and returns a JWT token for the user.
    async fn authenticate<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, AuthenticateRequest>,
    ) -> ServiceResult<OwnedAuthenticateResponseView> {
        log::debug!("Authenticating user with Google token");

        // Extract the Google token from the request
        let google_token = request.to_owned_message().google_token;

        // Verify the Google token
        match self.verify_token(&google_token).await {
            Ok(token_info) => {
                log::info!(
                    "Google token verified for user: {} ({})",
                    token_info.email,
                    token_info.sub
                );

                // Check if user exists in database, create if not
                let user_id = match self.find_or_create_user(&token_info).await {
                    Ok(id) => id,
                    Err(e) => {
                        log::error!("Failed to find or create user: {}", e);
                        return Err(connectrpc::ConnectError::internal(format!(
                            "Database error: {}",
                            e
                        )));
                    }
                };

                // Create custom claims with user information from Google
                let custom_claims = UserClaims::new(
                    Some(token_info.email.clone()),
                    token_info.name.clone(),
                    Some(token_info.sub.clone()),
                );

                // Sign a JWT token for the user
                // Use database user ID as the subject
                let jwt_token = self
                    .jwt_manager
                    .sign(user_id.clone(), custom_claims)
                    .map_err(|e| {
                        log::error!("Failed to sign JWT token: {}", e);
                        connectrpc::ConnectError::internal(e.to_string())
                    })?;

                log::debug!("JWT token created for user: {}", user_id);

                // Return the JWT token and user information
                let response = AuthenticateResponse {
                    jwt_token,
                    user_id,
                    email: token_info.email,
                    name: token_info.name.unwrap_or_default(),
                    ..Default::default()
                };
                let view = OwnedAuthenticateResponseView::from_owned(&response)
                    .map_err(|e| {
                        log::error!("Failed to create response view: {}", e);
                        connectrpc::ConnectError::internal(e.to_string())
                    })?;
                Ok(connectrpc::Response::new(view))
            }
            Err(e) => {
                log::warn!("Google token verification failed: {}", e);
                Err(connectrpc::ConnectError::invalid_argument(format!(
                    "Invalid Google token: {}",
                    e
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bb8_postgres::PostgresConnectionManager;
    use tokio_postgres::NoTls;

    #[tokio::test]
    async fn test_auth_service_creation() {
        let jwt_manager = JwtManager::new("test-secret".to_string(), chrono::Duration::hours(1));
        let mgr = PostgresConnectionManager::new_from_stringlike("", NoTls).unwrap();
        let db_pool = Pool::builder().build(mgr).await.unwrap();
        let service = AuthService::new("test-client-id".to_string(), jwt_manager, db_pool);
        assert_eq!(service.client_id, "test-client-id");
    }
}
