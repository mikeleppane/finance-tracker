// Application services/use cases

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::infrastructure::persistence::user_repository_cosmosdb::CosmosDbUserRepository;
        use color_eyre::Result;
        use std::sync::Arc;
        use crate::domain::models::user::User;
        use crate::domain::repositories::user_repository::UserRepository;
        use async_trait::async_trait;
        use crate::infrastructure::auth::jwt_service::AuthService;
        use crate::domain::models::user::CreateUserRequest;
        use crate::domain::models::user::AuthResponse;
        use axum::Json;
        use leptos::logging;
        use crate::application::errors::user_service_errors::UserServiceError;
        use crate::domain::models::user::UserProfile;


        #[async_trait]
        pub trait UserService<'a>: UserRepository + Send + Sync + 'static {
            async fn register_user(&self, user: CreateUserRequest, secret: &str) -> Result<Json<AuthResponse>, UserServiceError>;
            async fn authenticate_user(&self, email: &str, password: &str, secret: &str) -> Result<Json<AuthResponse>, UserServiceError>;
        }
        pub struct UserServiceImpl {
            user_repository: Arc<CosmosDbUserRepository>,
        }
        impl UserServiceImpl {
            #[must_use]
            pub fn new(user_repository: Arc<CosmosDbUserRepository>) -> Self {
                Self { user_repository }
            }
        }

        #[async_trait]
        impl UserService<'_> for UserServiceImpl {
            async fn register_user(&self, user: CreateUserRequest, secret: &str) -> Result<Json<AuthResponse>, UserServiceError> {

                logging::log!("Registering user: {}", user.email);

                // Check if user already exists
                match self.get_user_by_email(&user.email).await {
                    Ok(Some(_)) => {
                        return Err(UserServiceError::UserAlreadyExists {
                            email: user.email
                        });
                    },
                    Ok(None) => {},
                    Err(e) => {
                        return Err(UserServiceError::RepositoryError {
                            operation: "check_user_exists".to_string(),
                            source: Box::new(std::io::Error::other(e.to_string())),
                        });
                    }
                }

                // Hash password
                let hash = AuthService::hash_password(&user.password)
                    .map_err(|e| UserServiceError::AuthServiceError {
                        source: Box::new(e),
                    })?;

                let user = User::new(
                    &user.email,
                    hash,
                    &user.first_name,
                    &user.last_name,
                )?;

                // Create user
                self.create_user(user.clone()).await
                    .map_err(|e| UserServiceError::RepositoryError {
                        operation: "create_user".to_string(),
                        source: Box::new(std::io::Error::other(e.to_string())),
                    })?;

                logging::log!("User registered successfully: {}", user.email().as_str());

                        let token_pair = AuthService::generate_token_pair(&user, secret)
                    .map_err(|e| UserServiceError::AuthServiceError {
                        source: Box::new(e),
                    })?;

                let user_profile = UserProfile {
                    id: user.id().to_string(),
                    email: user.email().to_string(),
                    first_name: user.first_name().to_string(),
                    last_name: user.last_name().to_string(),
                };

                Ok(axum::Json(AuthResponse {
                    access_token: token_pair.access_token,
                    refresh_token: token_pair.refresh_token,
                    token_type: "Bearer".to_string(),
                    expires_in: token_pair.expires_in,
                    user: user_profile,
                }))
            }

            async fn authenticate_user(&self, email: &str, password: &str, secret: &str) -> Result<Json<AuthResponse>, UserServiceError> {
                // Find user by email

                logging::log!("Authenticating user: {}", email);

                let user = match self.get_user_by_email(email).await {
                    Ok(Some(user)) => user,
                    Ok(None) => {
                        return Err(UserServiceError::UserNotFound {
                            email: email.to_string(),
                        });
                    },
                    Err(e) => {
                        return Err(UserServiceError::RepositoryError {
                            operation: "get_user_by_email".to_string(),
                            source: Box::new(std::io::Error::other(e.to_string())),
                        });
                    }
                };

                // Verify password
                if AuthService::verify_password(password, user.password_hash().as_str()).is_err() {
                    return Err(UserServiceError::InvalidCredentials);
                }

                logging::log!("User authenticated successfully: {}", user.email().as_str());

                let token_pair = AuthService::generate_token_pair(&user, secret)
                    .map_err(|e| UserServiceError::AuthServiceError {
                        source: Box::new(e),
                    })?;

                let user_profile = UserProfile {
                    id: user.id().to_string(),
                    email: user.email().to_string(),
                    first_name: user.first_name().to_string(),
                    last_name: user.last_name().to_string(),
                };

                Ok(axum::Json(AuthResponse {
                    access_token: token_pair.access_token,
                    refresh_token: token_pair.refresh_token,
                    token_type: "Bearer".to_string(),
                    expires_in: token_pair.expires_in,
                    user: user_profile,
                }))
            }
        }

        // Implement the UserRepository trait for UserSer

        #[async_trait]
        impl UserRepository for UserServiceImpl {
            async fn create_user(&self, user: User) -> Result<()> {
                self.user_repository.create_user(user).await
            }

            async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
                self.user_repository.get_user_by_email(email).await
            }    // Additional methods can be implemented as needed
        }

        // CLone implementation for UserServiceImpl
        impl Clone for UserServiceImpl {
            fn clone(&self) -> Self {
                Self {
                    user_repository: Arc::clone(&self.user_repository),
                }
            }
        }
    }
}
