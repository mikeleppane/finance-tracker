use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::application::errors::user_service_errors::UserServiceError;
        use axum::http::StatusCode;
        use axum::response::{IntoResponse, Response};
        use axum::Json;
        use serde_json::json;
        use thiserror::Error;

        #[derive(Error, Debug)]
        pub enum WebError {
            #[error("User service error")]
            UserService {
                #[from]
                source: UserServiceError,
            },

            #[error("Invalid request format")]
            InvalidRequest { message: String },

            #[error("Missing authorization header")]
            MissingAuth,

            #[error("Invalid token")]
            InvalidToken,

            #[error("Request timeout")]
            Timeout,
        }

        impl WebError {
            #[must_use]
            pub fn to_status_code(&self) -> StatusCode {
                match self {
                    WebError::UserService { source } => match source {
                        UserServiceError::UserAlreadyExists { .. } => StatusCode::CONFLICT,
                        UserServiceError::UserNotFound { .. } => StatusCode::NOT_FOUND,
                        UserServiceError::InvalidCredentials => StatusCode::UNAUTHORIZED,
                        UserServiceError::DomainValidation { .. } => StatusCode::BAD_REQUEST,
                        UserServiceError::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
                        _ => StatusCode::INTERNAL_SERVER_ERROR,
                    },
                    WebError::InvalidRequest { .. } => StatusCode::BAD_REQUEST,
                    WebError::MissingAuth | WebError::InvalidToken => StatusCode::UNAUTHORIZED,
                    WebError::Timeout => StatusCode::REQUEST_TIMEOUT,
                }
            }

            #[must_use]
            pub fn user_message(&self) -> String {
                match self {
                    WebError::UserService { source } => match source {
                        UserServiceError::UserAlreadyExists { .. } => {
                            "An account with this email already exists".to_string()
                        }
                        UserServiceError::UserNotFound { .. } | UserServiceError::InvalidCredentials => {
                            "Invalid email or password".to_string()
                        }
                        UserServiceError::DomainValidation { source } => source.to_string(),
                        UserServiceError::ServiceUnavailable => {
                            "Service temporarily unavailable. Please try again later".to_string()
                        }
                        _ => "An error occurred. Please try again".to_string(),
                    },
                    WebError::InvalidRequest { message } => message.clone(),
                    WebError::MissingAuth => "Authentication required".to_string(),
                    WebError::InvalidToken => "Invalid authentication token".to_string(),
                    WebError::Timeout => "Request timed out. Please try again".to_string(),
                }
            }
        }

        impl IntoResponse for WebError {
            fn into_response(self) -> Response {
                let status = self.to_status_code();
                let message = self.user_message();

                let body = Json(json!({
                    "error": message,
                    "code": status.as_u16()
                }));

                (status, body).into_response()
            }
        }
    }
}
