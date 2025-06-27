use color_eyre::{Result, eyre::WrapErr};
use dotenvy;
use leptos::logging::log;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub cosmos: CosmosConfig,
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmosConfig {
    pub uri: String,
    pub database_name: String,
    pub containers: HashMap<String, ContainerConfig>,
    pub primary_key: String, // For development only
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerConfig {
    pub name: String,
    pub partition_key: String,
    pub throughput: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
}

impl AppConfig {
    /// Initializes environment and loads configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Required environment variables are not set
    /// - .env file cannot be loaded in development mode
    pub fn init() -> Result<Self> {
        // Load environment variables based on environment
        Self::load_environment()?;

        // Create configuration from environment variables
        Self::from_env()
    }
    /// Loads environment variables based on the current environment
    ///
    /// # Errors
    ///
    /// This function will return an error if the .env file cannot be loaded in development mode
    fn load_environment() -> Result<()> {
        let binding = std::env::var("ENVIRONMENT");
        let env = binding.as_deref().map(str::to_lowercase);

        if let Ok(env) = env {
            if env == "production" || env == "prod" {
                log!("🚀 Running in production mode - using system environment variables");
                // Don't load .env file in production
            } else {
                log!("🐛 Running in development mode - loading .env file");
                dotenvy::dotenv().wrap_err(
                    "Failed to load .env file. Make sure you have a .env file in the root directory."
                )?;
            }
        } else {
            log!("🐛 Running in development mode - loading .env file");
            dotenvy::dotenv().wrap_err(
                "Failed to load .env file. Make sure you have a .env file in the root directory.",
            )?;
        }

        Ok(())
    }

    /// Creates an `AppConfig` from environment variables.
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the required environment variables
    /// are not set:
    /// - `COSMOS_DB_URI` - The URI for the Cosmos DB instance
    /// - `COSMOS_DB_DATABASE` - The name of the Cosmos database
    /// - `COSMOS_DB_KEY` - The primary key for Cosmos DB access
    /// - `JWT_SECRET` - The secret key for JWT authentication
    pub fn from_env() -> Result<Self> {
        let cosmos_db_uri = std::env::var("COSMOS_DB_URI")
            .wrap_err("COSMOS_DB_URI environment variable not set")?;

        let cosmos_database_name = std::env::var("COSMOS_DB_DATABASE")
            .wrap_err("COSMOS_DB_DATABASE environment variable not set")?;

        let cosmos_primary_key = std::env::var("COSMOS_DB_KEY").wrap_err(
            "COSMOS_DB_KEY environment variable not set. This is required for development.",
        )?;

        let jwt_secret = std::env::var("JWT_SECRET").wrap_err(
            "JWT_SECRET environment variable not set. This is required for authentication.",
        )?;

        // Ensure the primary key is not empty

        // Define container configurations
        let mut containers = HashMap::new();

        // Blog posts container
        containers.insert(
            "users".to_string(),
            ContainerConfig {
                name: std::env::var("COSMOS_USERS_CONTAINER_NAME")
                    .unwrap_or_else(|_| "users".to_string()),
                partition_key: std::env::var("COSMOS_USERS_CONTAINER_NAME")
                    .unwrap_or_else(|_| "users".to_string()),
                throughput: Some(
                    std::env::var("COSMOS_USERS_CONTAINER_THROUGHPUT")
                        .unwrap_or_else(|_| "400".to_string())
                        .parse()
                        .unwrap_or(400),
                ),
            },
        );

        // Add more containers as needed
        // containers.insert("users".to_string(), ContainerConfig {
        //     name: std::env::var("COSMOS_USERS_CONTAINER_NAME")
        //         .unwrap_or_else(|_| "users".to_string()),
        //     partition_key: "/userId".to_string(),
        //     throughput: Some(400),
        // });

        let cosmos_config = CosmosConfig {
            uri: cosmos_db_uri,
            database_name: cosmos_database_name,
            containers,
            primary_key: cosmos_primary_key,
        };

        let server_config = ServerConfig {
            host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
        };

        Ok(AppConfig {
            cosmos: cosmos_config,
            server: server_config,
            auth: AuthConfig { jwt_secret },
        })
    }

    #[must_use]
    pub fn get_container_config(&self, container_type: &str) -> Option<&ContainerConfig> {
        self.cosmos.containers.get(container_type)
    }
}

static APP_CONFIG: std::sync::LazyLock<Result<AppConfig>> =
    std::sync::LazyLock::new(AppConfig::init);

/// Returns a reference to the global application configuration.
///
/// # Panics
///
/// This function will panic if the application configuration cannot be loaded,
/// which happens when required environment variables are not set or .env file
/// cannot be loaded in development mode.
pub fn get_config() -> &'static AppConfig {
    match APP_CONFIG.as_ref() {
        Ok(config) => config,
        Err(e) => {
            panic!(
                "{}",
                format!(
                    "Failed to load application configuration. Please ensure all required environment variables are set. Error:\n{e}"
                )
            );
        }
    }
}

/// Initializes the application configuration.
///
/// # Errors
///
/// This function will return an error if the configuration cannot be loaded,
/// which can happen when required environment variables are not set.
pub fn init_config() -> Result<()> {
    get_config();
    Ok(())
}
