use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::application::user_service::UserService;
        use crate::infrastructure::config::app_config::AppConfig;

        pub struct AppState<T: for<'a> UserService<'a> + Send + Sync + 'static> {
            user_service: T,
            app_config: AppConfig,
        }

        impl<T: for<'a> UserService<'a> + Send + Sync + 'static> AppState<T> {
            pub fn new(user_service: T, app_config: AppConfig) -> Self {
                Self {
                    user_service,
                    app_config,
                }
            }

            pub fn user_service(&self) -> &T {
                &self.user_service
            }

            pub fn app_config(&self) -> &AppConfig {
                &self.app_config
            }
        }
        impl<T: for<'a> UserService<'a> + Clone + Send + Sync> Clone for AppState<T> {
            fn clone(&self) -> Self {
                Self {
                    user_service: self.user_service.clone(),
                    app_config: self.app_config.clone(),
                }
            }
        }
    }
}
