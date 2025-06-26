use crate::domain::errors::user_errors::UserDomainError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserServiceError {
    // Business logic errors
    #[error("User with email '{email}' already exists")]
    UserAlreadyExists { email: String },

    #[error("User with email '{email}' not found")]
    UserNotFound { email: String },

    #[error("Invalid credentials provided")]
    InvalidCredentials,

    #[error("User registration failed")]
    RegistrationFailed,

    #[error("Authentication failed")]
    AuthenticationFailed,

    // Domain validation errors
    #[error("Domain validation failed")]
    DomainValidation { source: UserDomainError },

    // Infrastructure errors
    #[error("Repository operation failed: {operation}")]
    RepositoryError {
        operation: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Authentication service error")]
    AuthServiceError {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    // System errors
    #[error("Service temporarily unavailable")]
    ServiceUnavailable,

    #[error("Internal server error: {message}")]
    InternalError { message: String },
}

impl UserServiceError {
    /// Check if error is caused by user input
    #[must_use]
    pub fn is_user_error(&self) -> bool {
        matches!(
            self,
            UserServiceError::UserAlreadyExists { .. }
                | UserServiceError::InvalidCredentials
                | UserServiceError::DomainValidation { .. }
        )
    }

    /// Check if error is transient and retry might work
    #[must_use]
    pub fn is_transient(&self) -> bool {
        matches!(
            self,
            UserServiceError::ServiceUnavailable | UserServiceError::RepositoryError { .. }
        )
    }
}
impl From<UserDomainError> for UserServiceError {
    fn from(error: UserDomainError) -> Self {
        UserServiceError::DomainValidation { source: error }
    }
}
