use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::{
            application::user_service::UserService, domain::models::app_state::AppState,
            infrastructure::web::api::auth::auth_routes,
        };
        use axum::{routing::get, Router};
        use std::sync::Arc;

        pub fn create_api_router<T>(app_state: AppState<T>) -> Router
        where
            T: for<'a> UserService<'a> + Clone + Send + Sync + 'static,
        {

            let shared_state = Arc::new(app_state);

            Router::new()
                .nest("/auth", auth_routes(Arc::clone(&shared_state)))
                .route("/health", get(health_check))
        }

        async fn health_check() -> &'static str {
            "OK"
        }
    }
}
