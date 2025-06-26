use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHash(String);

impl PasswordHash {
    /// Creates a new `PasswordHash` from a hash string.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The hash is empty or contains only whitespace
    /// - The hash is shorter than 8 characters
    /// - The hash is longer than 255 characters
    /// - The hash doesn't have a valid bcrypt format (doesn't start with "$2")
    pub fn new(hash: String) -> Result<Self, PasswordHashError> {
        if hash.trim().is_empty() {
            return Err(PasswordHashError::Empty);
        }

        if hash.len() < 8 {
            return Err(PasswordHashError::TooShort);
        }

        if hash.len() > 255 {
            return Err(PasswordHashError::TooLong);
        }

        // Basic check for bcrypt format
        if !hash.starts_with("$2") {
            return Err(PasswordHashError::InvalidFormat);
        }

        Ok(Self(hash))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordHashError {
    #[error("Password hash cannot be empty")]
    Empty,
    #[error("Password hash is too short")]
    TooShort,
    #[error("Password hash is too long (max 255 characters)")]
    TooLong,
    #[error("Invalid password hash format")]
    InvalidFormat,
}

impl fmt::Display for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}
