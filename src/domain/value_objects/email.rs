use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    /// Creates a new Email instance from a string.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The email is empty after trimming
    /// - The email is longer than 254 characters
    /// - The email format is invalid
    pub fn new(email: &str) -> Result<Self, EmailError> {
        let email = email.trim().to_lowercase();

        if email.is_empty() {
            return Err(EmailError::Empty);
        }

        if email.len() > 254 {
            return Err(EmailError::TooLong);
        }

        if !Self::is_valid_format(&email) {
            return Err(EmailError::InvalidFormat);
        }

        Ok(Self(email))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[allow(clippy::expect_used)]
    fn is_valid_format(email: &str) -> bool {
        // Basic email validation regex
        let email_regex =
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").expect("Invalid regex");
        // Check if the email matches the regex
        email_regex.is_match(email)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for Email {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Email cannot be empty")]
    Empty,
    #[error("Email is too long (max 254 characters)")]
    TooLong,
    #[error("Invalid email format")]
    InvalidFormat,
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
