#![allow(clippy::expect_used)]
#[cfg(feature = "ssr")]
use color_eyre::Result;
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use axum;
    use axum::Router;
    use axum::routing::get;
    use finance_tracker::app::{App, shell};
    use finance_tracker::application::user_service::UserServiceImpl;
    use finance_tracker::domain::models::app_state::AppState;
    use finance_tracker::infrastructure::config::app_config::get_config;
    use finance_tracker::infrastructure::persistence::user_repository_cosmosdb::CosmosDbUserRepository;
    use finance_tracker::infrastructure::web::routing::app_router::create_api_router;
    use finance_tracker::infrastructure::web::websocket::{ConnectionManager, websocket_handler};
    use leptos::logging;
    use leptos::prelude::*;
    use leptos_axum;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use std::sync::Arc;
    use tokio;

    let conf = get_configuration(None).expect("Failed to load configuration");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // Create connection manager for WebSockets
    let connection_manager = Arc::new(ConnectionManager::new());

    // Start heartbeat service in background
    let heartbeat_connection_manager = connection_manager.clone();
    tokio::spawn(async move {
        use finance_tracker::infrastructure::web::websocket::start_heartbeat_service;

        start_heartbeat_service(heartbeat_connection_manager).await;
    });

    let app_config = get_config();

    let user_repo = CosmosDbUserRepository::new(
        app_config.cosmos.database_name.clone(),
        app_config
            .get_container_config("users")
            .expect("Container config for 'users' not found")
            .name
            .clone(),
        &app_config.cosmos.uri,
        app_config.cosmos.primary_key.clone(),
    );

    let user_service = UserServiceImpl::new(Arc::new(user_repo));

    let app_state = AppState::new(user_service, app_config.clone());

    let api_router = create_api_router(app_state);

    let ws_router = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(connection_manager);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        .merge(Router::new().nest("/api", api_router))
        .merge(ws_router);
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    logging::log!(" listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind TCP listener");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
