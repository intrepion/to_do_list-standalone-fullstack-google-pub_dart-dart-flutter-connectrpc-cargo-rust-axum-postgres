//! Authentication routes for Google OAuth token verification

use axum::{
    extract::State,
    http::StatusCode,
    Json, Router,
};
use std::sync::Arc;

use crate::{
    server::AppState,
    services::auth::{AuthService, TokenVerificationRequest, TokenVerificationResponse},
};

/// Create router for authentication routes
pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/verify-token", axum::routing::post(verify_google_token))
}

/// Verify a Google OAuth ID token
/// 
/// This endpoint accepts a POST request with an ID token in the request body
/// and verifies it with Google's token info endpoint.
/// 
/// # Request
/// ```json
/// {
///     "id_token": "ya29.a0Ae4..."
/// }
/// ```
/// 
/// # Response
/// ```json
/// {
///     "valid": true,
///     "google_id": "123456789",
///     "email": "user@example.com",
///     "display_name": "John Doe",
///     "error": null
/// }
/// ```
#[axum::debug_handler]
async fn verify_google_token(
    State(state): State<Arc<AppState>>,
    Json(request): Json<TokenVerificationRequest>,
) -> Result<Json<TokenVerificationResponse>, StatusCode> {
    log::debug!("Verifying Google token");

    // Create auth service with client ID from config
    // Use new_for_verification since we don't need JWT signing for this endpoint
    let auth_service = AuthService::new_for_verification(state.config.google_oauth_client_id.clone());

    // Verify the token
    // Use the existing auth service with dummy JWT manager (not used for verification)
    let response = auth_service.verify_and_response(&request.id_token).await;

    // Log the result
    if response.valid {
        let email = response.email.clone().unwrap_or_else(|| "unknown".to_string());
        log::info!("Token verified successfully for user: {}", email);
    } else {
        let error = response.error.clone().unwrap_or_else(|| "unknown error".to_string());
        log::warn!("Token verification failed: {}", error);
    }

    Ok(Json(response))
}
