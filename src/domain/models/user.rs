use crate::domain::{
    errors::user_errors::UserDomainError,
    value_objects::{Email, PasswordHash, UserId, UserName},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    email: Email,
    password_hash: PasswordHash,
    first_name: UserName,
    last_name: UserName,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    /// Create a new user with validation
    ///
    /// # Errors
    ///
    /// Returns `UserCreationError` if any of the provided parameters are invalid:
    /// - `InvalidEmail` if the email format is invalid
    /// - `InvalidPasswordHash` if the password hash is invalid
    /// - `InvalidFirstName` if the first name is invalid
    /// - `InvalidLastName` if the last name is invalid
    pub fn new(
        email: &str,
        password_hash: String,
        first_name: &str,
        last_name: &str,
    ) -> Result<Self, UserDomainError> {
        let now = Utc::now();

        Ok(Self {
            id: UserId::generate(),
            email: Email::new(email)?,
            password_hash: PasswordHash::new(password_hash)?,
            first_name: UserName::new(first_name)?,
            last_name: UserName::new(last_name)?,
            created_at: now,
            updated_at: now,
        })
    }

    #[must_use]
    pub fn email(&self) -> &Email {
        &self.email
    }

    #[must_use]
    pub fn id(&self) -> &UserId {
        &self.id
    }

    #[must_use]
    pub fn first_name(&self) -> &UserName {
        &self.first_name
    }
    #[must_use]
    pub fn last_name(&self) -> &UserName {
        &self.last_name
    }
    #[must_use]
    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user: UserProfile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email.to_string(),
            first_name: user.first_name.to_string(),
            last_name: user.last_name.to_string(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserCreationError {
    #[error("Invalid user ID: {0}")]
    InvalidUserId(#[from] crate::domain::value_objects::user_id::UserIdError),
    #[error("Invalid email: {0}")]
    InvalidEmail(#[from] crate::domain::value_objects::email::EmailError),
    #[error("Invalid password hash: {0}")]
    InvalidPasswordHash(#[from] crate::domain::value_objects::password_hash::PasswordHashError),
    #[error("Invalid first name: {0}")]
    InvalidFirstName(#[from] crate::domain::value_objects::user_name::UserNameError),
    #[error("Invalid last name: {0}")]
    InvalidLastName(crate::domain::value_objects::user_name::UserNameError),
}

#[derive(Debug, thiserror::Error)]
pub enum UserUpdateError {
    #[error("Invalid email: {0}")]
    InvalidEmail(#[from] crate::domain::value_objects::email::EmailError),
    #[error("Invalid password hash: {0}")]
    InvalidPasswordHash(#[from] crate::domain::value_objects::password_hash::PasswordHashError),
    #[error("Invalid first name: {0}")]
    InvalidFirstName(#[from] crate::domain::value_objects::user_name::UserNameError),
    #[error("Invalid last name: {0}")]
    InvalidLastName(crate::domain::value_objects::user_name::UserNameError),
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User {{ id: {}, email: {}, first_name: {}, last_name: {}, created_at: {}, updated_at: {} }}",
            self.id, self.email, self.first_name, self.last_name, self.created_at, self.updated_at
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserCreationRequestError {
    #[error("Email is required and cannot be empty")]
    EmailRequired,
    #[error("Email format is invalid")]
    InvalidEmail,
    #[error("Password is required and must be at least 8 characters")]
    InvalidPassword,
    #[error("First name is required and cannot be empty")]
    FirstNameRequired,
    #[error("Last name is required and cannot be empty")]
    LastNameRequired,
    #[error("Password hashing failed: {0}")]
    PasswordHashingFailed(String),
    #[error("User creation failed: {0}")]
    UserCreationFailed(#[from] UserCreationError),
}
