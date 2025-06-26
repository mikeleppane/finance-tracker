use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::{
            application::user_service::UserService, domain::models::app_state::AppState,
            infrastructure::web::api::auth::auth_routes,
        };
        use axum::{routing::get, Router};

        pub fn create_api_router<T>(app_state: AppState<T>) -> Router
        where
            T: UserService + Clone + Send + Sync + 'static,
        {
            Router::new()
                .nest("/auth", auth_routes(app_state))
                .route("/health", get(health_check))
        }

        async fn health_check() -> &'static str {
            "OK"
        }
    }
}
