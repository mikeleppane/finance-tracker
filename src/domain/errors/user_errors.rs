use thiserror::Error;

use crate::domain::value_objects::{
    email::EmailError, password_hash::PasswordHashError, user_id::UserIdError,
    user_name::UserNameError,
};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum UserDomainError {
    #[error("Invalid email format: {email}")]
    InvalidEmail { email: String },

    #[error("Password does not meet security requirements: {message}")]
    WeakPassword { message: String },

    #[error("First name cannot be empty")]
    EmptyFirstName,

    #[error("Last name cannot be empty")]
    EmptyLastName,

    #[error("User ID is invalid")]
    InvalidUserId,

    #[error("Email address is required")]
    EmailRequired,

    #[error("Name contains invalid characters")]
    InvalidNameFormat,
}

impl From<EmailError> for UserDomainError {
    fn from(email_error: EmailError) -> Self {
        match email_error {
            EmailError::Empty => UserDomainError::EmailRequired,
            EmailError::TooLong => UserDomainError::InvalidEmail {
                email: "Email too long".to_string(),
            },
            EmailError::InvalidFormat => UserDomainError::InvalidEmail {
                email: "Invalid format".to_string(),
            },
        }
    }
}

impl From<PasswordHashError> for UserDomainError {
    fn from(err: PasswordHashError) -> Self {
        UserDomainError::WeakPassword {
            message: err.to_string(),
        }
    }
}

impl From<UserIdError> for UserDomainError {
    fn from(_err: UserIdError) -> Self {
        UserDomainError::InvalidUserId
    }
}

impl From<UserNameError> for UserDomainError {
    fn from(err: UserNameError) -> Self {
        match err {
            UserNameError::Empty => UserDomainError::EmptyFirstName,
            UserNameError::TooShort | UserNameError::TooLong | UserNameError::ContainsNumbers => {
                UserDomainError::InvalidNameFormat
            }
            UserNameError::InvalidCharacters => UserDomainError::InvalidNameFormat,
        }
    }
}

impl UserDomainError {
    #[must_use]
    pub fn is_validation_error(&self) -> bool {
        matches!(
            self,
            UserDomainError::InvalidEmail { .. }
                | UserDomainError::WeakPassword { .. }
                | UserDomainError::EmptyFirstName
                | UserDomainError::EmptyLastName
                | UserDomainError::EmailRequired
                | UserDomainError::InvalidNameFormat
        )
    }
}
