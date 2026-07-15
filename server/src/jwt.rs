//! JWT (JSON Web Token) signing and verification utilities

use anyhow::{anyhow, Context, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// JWT claims structure with standard fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims<T> {
    /// Subject (typically user ID)
    pub sub: String,
    /// Issued at timestamp
    pub iat: i64,
    /// Expiration timestamp
    pub exp: i64,
    /// Issuer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    /// Audience
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
    /// Custom claims
    #[serde(flatten)]
    pub custom: T,
}

impl<T> JwtClaims<T> {
    /// Create new JWT claims with current timestamp
    pub fn new(sub: String, custom: T, expires_in: Duration) -> Self {
        let now = Utc::now().timestamp();
        Self {
            sub,
            iat: now,
            exp: now + expires_in.num_seconds(),
            iss: None,
            aud: None,
            custom,
        }
    }

    /// Create new JWT claims with issuer and audience
    pub fn with_issuer_audience(
        sub: String,
        custom: T,
        expires_in: Duration,
        iss: Option<String>,
        aud: Option<String>,
    ) -> Self {
        let now = Utc::now().timestamp();
        Self {
            sub,
            iat: now,
            exp: now + expires_in.num_seconds(),
            iss,
            aud,
            custom,
        }
    }

    /// Validate that the token hasn't expired
    pub fn validate_expiration(&self) -> Result<()> {
        let now = Utc::now().timestamp();
        if now > self.exp {
            return Err(anyhow!("Token has expired"));
        }
        Ok(())
    }
}

/// JWT utility struct for signing and verifying tokens
#[derive(Clone)]
pub struct JwtManager {
    /// Secret key for HMAC signing/verification
    secret: String,
    /// Issuer (optional)
    issuer: Option<String>,
    /// Audience (optional)
    audience: Option<String>,
    /// Token expiration duration
    expires_in: Duration,
}

impl JwtManager {
    /// Create a new JWT manager with the given secret
    ///
    /// # Arguments
    /// * `secret` - The secret key for HMAC signing
    /// * `expires_in` - Token expiration duration (e.g., Duration::hours(24))
    pub fn new(secret: String, expires_in: Duration) -> Self {
        Self {
            secret,
            issuer: None,
            audience: None,
            expires_in,
        }
    }

    /// Create a new JWT manager with issuer and audience
    pub fn with_claims(
        secret: String,
        expires_in: Duration,
        issuer: Option<String>,
        audience: Option<String>,
    ) -> Self {
        Self {
            secret,
            issuer,
            audience,
            expires_in,
        }
    }

    /// Sign a token with custom claims
    ///
    /// # Arguments
    /// * `sub` - Subject (typically user ID)
    /// * `custom_claims` - Custom claims to include in the token
    ///
    /// # Returns
    /// A signed JWT token string
    pub fn sign<T: Serialize>(&self, sub: String, custom_claims: T) -> Result<String> {
        let now = Utc::now();
        let claims = JwtClaims {
            sub,
            iat: now.timestamp(),
            exp: (now + self.expires_in).timestamp(),
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
            custom: custom_claims,
        };

        let encoding_key = EncodingKey::from_secret(self.secret.as_ref());
        
        encode(&Header::default(), &claims, &encoding_key)
            .context("Failed to encode JWT token")
    }

    /// Sign a token with empty custom claims
    pub fn sign_simple(&self, sub: String) -> Result<String> {
        self.sign(sub, ())
    }

    /// Verify a token and extract claims
    ///
    /// # Arguments
    /// * `token` - The JWT token string to verify
    ///
    /// # Returns
    /// The decoded claims if verification succeeds
    pub fn verify<T: DeserializeOwned + std::fmt::Debug>(&self, token: &str) -> Result<JwtClaims<T>> {
        let decoding_key = DecodingKey::from_secret(self.secret.as_ref());
        
        let mut validation = Validation::new(self.get_algorithm());
        
        // Set issuer validation if configured
        if let Some(ref issuer) = self.issuer {
            validation.set_issuer(&[issuer.as_str()]);
        }
        
        // Set audience validation if configured
        if let Some(ref audience) = self.audience {
            validation.set_audience(&[audience.as_str()]);
        }
        
        // Validate expiration by default
        validation.validate_exp = true;
        validation.validate_nbf = true;
        
        // Set leeway to 0 for strict expiration checking
        validation.leeway = 0;
        
        let token_data: TokenData<JwtClaims<T>> = decode(token, &decoding_key, &validation)
            .context("Failed to decode JWT token")?;
        
        Ok(token_data.claims)
    }

    /// Verify a token without custom claims
    pub fn verify_simple(&self, token: &str) -> Result<JwtClaims<()>> {
        self.verify(token)
    }

    /// Get the JWT algorithm (HS256 by default)
    fn get_algorithm(&self) -> jsonwebtoken::Algorithm {
        jsonwebtoken::Algorithm::HS256
    }

    /// Validate a token without decoding claims (for simple validation)
    pub fn validate_token(&self, token: &str) -> Result<bool> {
        match self.verify::<()>(token) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Get the secret key (for testing or debugging)
    pub fn secret(&self) -> &str {
        &self.secret
    }
}

/// Simple JWT claims for basic user authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    /// User's email address
    pub email: Option<String>,
    /// User's display name
    pub name: Option<String>,
    /// Google user ID
    pub google_id: Option<String>,
}

impl UserClaims {
    /// Create new user claims
    pub fn new(email: Option<String>, name: Option<String>, google_id: Option<String>) -> Self {
        Self {
            email,
            name,
            google_id,
        }
    }
}

/// JWT claims with user information
pub type UserJwtClaims = JwtClaims<UserClaims>;

/// Short-lived token for temporary operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenClaims;

/// Refresh token JWT claims
pub type RefreshJwtClaims = JwtClaims<RefreshTokenClaims>;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    const TEST_SECRET: &str = "test-secret-key-for-jwt-signing";
    const TEST_SUBJECT: &str = "test-user-id";

    #[test]
    fn test_jwt_roundtrip() {
        let jwt_manager = JwtManager::new(TEST_SECRET.to_string(), Duration::hours(1));
        
        // Test with empty custom claims
        let token = jwt_manager.sign_simple(TEST_SUBJECT.to_string()).unwrap();
        let claims: JwtClaims<()> = jwt_manager.verify_simple(&token).unwrap();
        
        assert_eq!(claims.sub, TEST_SUBJECT);
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_jwt_with_custom_claims() {
        let jwt_manager = JwtManager::new(TEST_SECRET.to_string(), Duration::hours(1));
        
        let custom_claims = UserClaims::new(
            Some("test@example.com".to_string()),
            Some("Test User".to_string()),
            Some("google-id-123".to_string()),
        );
        
        let token = jwt_manager.sign(TEST_SUBJECT.to_string(), custom_claims).unwrap();
        let claims: JwtClaims<UserClaims> = jwt_manager.verify(&token).unwrap();
        
        assert_eq!(claims.sub, TEST_SUBJECT);
        assert_eq!(claims.custom.email, Some("test@example.com".to_string()));
        assert_eq!(claims.custom.name, Some("Test User".to_string()));
        assert_eq!(claims.custom.google_id, Some("google-id-123".to_string()));
    }

    #[test]
    fn test_jwt_expiration() {
        // Create a JWT manager with 1 second expiration
        // Note: JWT timestamps are in seconds, so we need at least 1 second
        let jwt_manager = JwtManager::new(TEST_SECRET.to_string(), Duration::seconds(1));
        
        let token = jwt_manager.sign_simple(TEST_SUBJECT.to_string()).unwrap();
        
        // Wait for token to expire - sleep longer than the expiration time
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        // Verification should fail
        let result = jwt_manager.verify_simple(&token);
        assert!(result.is_err(), "Token should have expired: {:?}", result);
    }

    #[test]
    fn test_jwt_invalid_secret() {
        let jwt_manager = JwtManager::new(TEST_SECRET.to_string(), Duration::hours(1));
        let wrong_manager = JwtManager::new("wrong-secret".to_string(), Duration::hours(1));
        
        let token = jwt_manager.sign_simple(TEST_SUBJECT.to_string()).unwrap();
        
        // Verification with wrong secret should fail
        let result = wrong_manager.verify_simple(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_jwt_invalid_token_format() {
        let jwt_manager = JwtManager::new(TEST_SECRET.to_string(), Duration::hours(1));
        
        // Invalid token string
        let result = jwt_manager.verify_simple("invalid-token");
        assert!(result.is_err());
    }

    #[test]
    fn test_claims_creation() {
        let custom_claims = UserClaims::new(
            Some("test@example.com".to_string()),
            Some("Test User".to_string()),
            Some("google-id-123".to_string()),
        );
        
        let claims = JwtClaims::new(TEST_SUBJECT.to_string(), custom_claims, Duration::hours(1));
        
        assert_eq!(claims.sub, TEST_SUBJECT);
        assert_eq!(claims.custom.email, Some("test@example.com".to_string()));
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_claims_with_issuer_audience() {
        let custom_claims = UserClaims::new(None, None, None);
        
        let claims = JwtClaims::with_issuer_audience(
            TEST_SUBJECT.to_string(),
            custom_claims,
            Duration::hours(1),
            Some("test-issuer".to_string()),
            Some("test-audience".to_string()),
        );
        
        assert_eq!(claims.sub, TEST_SUBJECT);
        assert_eq!(claims.iss, Some("test-issuer".to_string()));
        assert_eq!(claims.aud, Some("test-audience".to_string()));
    }
}
