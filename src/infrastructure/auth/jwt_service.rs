#[cfg(feature = "ssr")]
use crate::domain::models::user::User;
#[cfg(feature = "ssr")]
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
#[cfg(feature = "ssr")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use thiserror::Error;

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
    pub token_type: TokenType,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    Access,
    Refresh,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64, // Access token expiration in seconds
}

#[cfg(feature = "ssr")]
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("Bcrypt error: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("Invalid token type: expected {expected}, got {actual}")]
    InvalidTokenType { expected: String, actual: String },

    #[error("Token has expired")]
    TokenExpired,

    #[error("Refresh token not found or invalid")]
    InvalidRefreshToken,
}

#[cfg(feature = "ssr")]
#[derive(Clone, Debug)]
pub struct AuthService {}

#[cfg(feature = "ssr")]
impl AuthService {
    const ACCESS_TOKEN_DURATION_MINUTES: u64 = 15; // 15 minutes
    const REFRESH_TOKEN_DURATION_DAYS: u64 = 30; // 30 days

    /// Generates a token pair (access + refresh tokens) for the given user.
    ///
    /// # Errors
    ///
    /// Returns an `AuthError` if token generation fails.
    pub fn generate_token_pair(user: &User, secret: &str) -> Result<TokenPair, AuthError> {
        let access_token = Self::generate_access_token(user, secret)?;
        let refresh_token = Self::generate_refresh_token(user, secret)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            expires_in: (Self::ACCESS_TOKEN_DURATION_MINUTES * 60),
        })
    }

    /// Generates a short-lived access token.
    fn generate_access_token(user: &User, secret: &str) -> Result<String, AuthError> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::minutes(
                Self::ACCESS_TOKEN_DURATION_MINUTES.try_into().unwrap_or(15),
            ))
            .ok_or_else(|| {
                AuthError::JwtError(jsonwebtoken::errors::Error::from(
                    jsonwebtoken::errors::ErrorKind::InvalidToken,
                ))
            })?
            .timestamp()
            .try_into()
            .map_err(|_| {
                AuthError::JwtError(jsonwebtoken::errors::Error::from(
                    jsonwebtoken::errors::ErrorKind::InvalidToken,
                ))
            })?;

        let claims = Claims {
            sub: user.id().to_string(),
            email: user.email().to_string(),
            exp: expiration,
            token_type: TokenType::Access,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(AuthError::from)
    }

    /// Generates a long-lived refresh token.
    fn generate_refresh_token(user: &User, secret: &str) -> Result<String, AuthError> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(
                Self::REFRESH_TOKEN_DURATION_DAYS.try_into().unwrap_or(30),
            ))
            .ok_or_else(|| {
                AuthError::JwtError(jsonwebtoken::errors::Error::from(
                    jsonwebtoken::errors::ErrorKind::InvalidToken,
                ))
            })?
            .timestamp()
            .try_into()
            .map_err(|_| {
                AuthError::JwtError(jsonwebtoken::errors::Error::from(
                    jsonwebtoken::errors::ErrorKind::InvalidToken,
                ))
            })?;

        let claims = Claims {
            sub: user.id().to_string(),
            email: user.email().to_string(),
            exp: expiration,
            token_type: TokenType::Refresh,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(AuthError::from)
    }

    /// Refreshes an access token using a valid refresh token.
    ///
    /// # Errors
    ///
    /// Returns an `AuthError` if the refresh token is invalid or if token generation fails.
    pub async fn refresh_access_token<T>(
        refresh_token: &str,
        secret: &str,
        user_service: &T,
    ) -> Result<TokenPair, AuthError>
    where
        T: crate::application::user_service::UserService,
    {
        // Verify the refresh token
        let claims = Self::verify_token(refresh_token, secret)?;

        // Ensure it's actually a refresh token
        if !matches!(claims.token_type, TokenType::Refresh) {
            return Err(AuthError::InvalidTokenType {
                expected: "Refresh".to_string(),
                actual: format!("{:?}", claims.token_type),
            });
        }

        // Get the user to ensure they still exist and are active
        let user = user_service
            .get_user_by_email(&claims.email)
            .await
            .map_err(|_| AuthError::InvalidRefreshToken)?
            .ok_or(AuthError::InvalidRefreshToken)?;

        // Generate new token pair
        Self::generate_token_pair(&user, secret)
    }

    /// Legacy method for backward compatibility - generates only access token.
    ///
    /// # Errors
    ///
    /// Returns a `jsonwebtoken::errors::Error` if the token encoding fails.
    pub fn generate_token(
        user: &User,
        secret: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        Self::generate_access_token(user, secret).map_err(|e| match e {
            AuthError::JwtError(jwt_err) => jwt_err,
            _ => jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken),
        })
    }

    /// Verifies a JWT token and extracts claims.
    ///
    /// # Errors
    ///
    /// Returns an `AuthError` if the token is invalid, expired, or malformed.
    pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AuthError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map(|data| data.claims)
        .map_err(AuthError::from)
    }

    /// Verifies an access token specifically.
    ///
    /// # Errors
    ///
    /// Returns an `AuthError` if the token is invalid or not an access token.
    pub fn verify_access_token(token: &str, secret: &str) -> Result<Claims, AuthError> {
        let claims = Self::verify_token(token, secret)?;

        if !matches!(claims.token_type, TokenType::Access) {
            return Err(AuthError::InvalidTokenType {
                expected: "Access".to_string(),
                actual: format!("{:?}", claims.token_type),
            });
        }

        Ok(claims)
    }

    /// Hashes a password using bcrypt with the default cost.
    ///
    /// # Errors
    ///
    /// Returns an `AuthError` if the hashing process fails.
    pub fn hash_password(password: &str) -> Result<String, AuthError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(AuthError::from)
    }

    /// Verifies if a password matches the provided hash.
    ///
    /// # Errors
    ///
    /// Returns an `AuthError` if the verification process fails.
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
        bcrypt::verify(password, hash).map_err(AuthError::from)
    }
}
