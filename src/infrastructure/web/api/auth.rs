use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::{domain::models::app_state::AppState};
        use axum::{extract::State, http::StatusCode, response::Json, routing::post, Router};
        use crate::application::user_service::UserService;
        use crate::domain::models::user::CreateUserRequest;
        use crate::domain::models::user::AuthResponse;
        use crate::domain::models::user::LoginRequest;
        use std::sync::Arc;
        use crate::domain::models::user::RefreshTokenRequest;
        use crate::domain::models::user::RefreshTokenResponse;
        use crate::infrastructure::auth::jwt_service::AuthService;
        use crate::infrastructure::errors::web_errors::WebError;
        use crate::application::errors::user_service_errors::UserServiceError;

        pub fn auth_routes<T>(app_state: AppState<T>) -> Router
        where
            T: UserService + Clone + Send + Sync + 'static,
         {
            Router::new()
                .route("/register", post(register_handler))
                .route("/login", post(login_handler))
                .route("/refresh", post(refresh_token_handler::<T>))
                .with_state(Arc::new(app_state))
        }
        async fn register_handler<T>(
            State(state): State<Arc<AppState<T>>>,
            Json(request): Json<CreateUserRequest>,
        ) -> Result<Json<AuthResponse>, StatusCode>
        where
            T: UserService + Clone + Send + Sync + 'static,
        {
            let secret = state.app_config().auth.jwt_secret.clone();

            // Your service should return Result<AuthResponse, Error>, not Result<Json<AuthResponse>, Error>
            let auth_response: Json<AuthResponse> = state
                .user_service()
                .register_user(request, secret)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Wrap the response in Json at the handler level
            Ok(auth_response)
        }
        async fn login_handler<T>(
            State(state): State<Arc<AppState<T>>>,
            Json(request): Json<LoginRequest>,
        ) -> Result<Json<AuthResponse>, StatusCode>
        where
            T: UserService + Clone + Send + Sync + 'static,

         {
            let secret = state.app_config().auth.jwt_secret.clone();
            let result = state
                .user_service()
                .authenticate_user(&request.email, &request.password, secret)
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?;
            Ok(result)
        }

        async fn refresh_token_handler<T>(
            State(state): State<Arc<AppState<T>>>,
            Json(request): Json<RefreshTokenRequest>,
        ) -> Result<Json<RefreshTokenResponse>, WebError>
        where
            T: UserService + Clone + Send + Sync + 'static,
        {
            let secret = state.app_config().auth.jwt_secret.clone();

            let token_pair = AuthService::refresh_access_token(
                &request.refresh_token,
                &secret,
                state.user_service(),
            )
            .await
            .map_err(|e| WebError::UserService {
                source: UserServiceError::AuthServiceError {
                    source: Box::new(e),
                },
            })?;

            Ok(Json(RefreshTokenResponse {
                access_token: token_pair.access_token,
                refresh_token: token_pair.refresh_token,
                token_type: "Bearer".to_string(),
                expires_in: token_pair.expires_in,
            }))
        }
    }
}
