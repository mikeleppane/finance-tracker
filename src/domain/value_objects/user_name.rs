use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserName(String);

impl UserName {
    /// Creates a new `UserName` from the provided string.
    ///
    /// # Errors
    ///
    /// Returns a `UserNameError` if the name:
    /// - Is empty after trimming whitespace
    /// - Is shorter than 2 characters
    /// - Is longer than 50 characters
    /// - Contains numeric characters
    /// - Contains invalid punctuation (only `-`, `'`, and spaces are allowed)
    pub fn new(name: &str) -> Result<Self, UserNameError> {
        let name = name.trim().to_string();

        if name.is_empty() {
            return Err(UserNameError::Empty);
        }

        if name.len() < 2 {
            return Err(UserNameError::TooShort);
        }

        if name.len() > 50 {
            return Err(UserNameError::TooLong);
        }

        if name.chars().any(char::is_numeric) {
            return Err(UserNameError::ContainsNumbers);
        }

        if name
            .chars()
            .any(|c| c.is_ascii_punctuation() && c != '-' && c != '\'' && c != ' ')
        {
            return Err(UserNameError::InvalidCharacters);
        }

        Ok(Self(name))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserNameError {
    #[error("Name cannot be empty")]
    Empty,
    #[error("Name is too short (min 2 characters)")]
    TooShort,
    #[error("Name is too long (max 50 characters)")]
    TooLong,
    #[error("Name cannot contain numbers")]
    ContainsNumbers,
    #[error("Name contains invalid characters")]
    InvalidCharacters,
}

impl fmt::Display for UserName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
