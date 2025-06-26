use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(String);

impl UserId {
    /// Creates a new `UserId` from the provided string.
    ///
    /// # Errors
    ///
    /// Returns `UserIdError::Empty` if the provided string is empty or contains only whitespace.
    /// Returns `UserIdError::TooLong` if the provided string is longer than 36 characters.
    pub fn new(id: String) -> Result<Self, UserIdError> {
        if id.trim().is_empty() {
            return Err(UserIdError::Empty);
        }

        if id.len() > 36 {
            return Err(UserIdError::TooLong);
        }

        Ok(Self(id))
    }

    #[must_use]
    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserIdError {
    #[error("User ID cannot be empty")]
    Empty,
    #[error("User ID is too long (max 36 characters)")]
    TooLong,
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
