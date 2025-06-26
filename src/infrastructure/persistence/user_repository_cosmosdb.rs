// Cosmos DB implementation of the UserRepository trait
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use azure_core::credentials::Secret;
        use azure_data_cosmos::clients::ContainerClient;
        use azure_data_cosmos::{CosmosClient, PartitionKey};
        use std::sync::Arc;
        use color_eyre::Result;
        use crate::domain::models::user::User;
        use crate::domain::repositories::user_repository::UserRepository;
        use async_trait::async_trait;
        use leptos::leptos_dom::logging;
        use futures_util::stream::TryStreamExt;

        pub struct CosmosDbUserRepository {
            client: Arc<CosmosClient>,
            database_name: String,
            container_name: String,
        }

        impl CosmosDbUserRepository {
            #[allow(clippy::missing_panics_doc)]
            #[must_use]
            pub fn new(
                database_name: String,
                container_name: String,
                uri: &str,
                key: String,
            ) -> Self {
                #[allow(clippy::expect_used)]
                let client = CosmosClient::with_key(uri, Secret::from(key), None)
                    .expect("Failed to create Cosmos client");
                Self {
                    client: Arc::new(client),
                    database_name,
                    container_name,
                }
            }

            #[must_use]
            pub fn get_container(&self) -> ContainerClient {
                self.client
                    .database_client(&self.database_name)
                    .container_client(&self.container_name)
            }
        }
        #[async_trait]
        impl UserRepository for CosmosDbUserRepository {
            async fn create_user(&self, user: User) -> Result<()> {
                let container = self.get_container();
                let partition_key = PartitionKey::from(user.email().to_string());
                container.create_item(partition_key, user, None).await?;
                Ok(())
            }
            async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
                let partition_key = PartitionKey::from(email.to_string().clone());

                let query = format!("SELECT * FROM c WHERE c.email = '{email}'");

                logging::console_log(&format!("Querying for user by email: {email}"));
                let query_result = self
                    .get_container()
                    .query_items::<User>(query, partition_key, None);

                match query_result {
                    Ok(mut query_stream) => {
                        let mut user = None;
                        if let Some(feed_page) = query_stream.try_next().await? {
                            logging::console_log(&format!(
                                "Received feed page with {} items",
                                feed_page.items().len()
                            ));

                            match feed_page.items().len() {
                                0 => {
                                    logging::console_warn(&format!("No user found for email: {email}"));
                                }
                                1 => {
                                    user = feed_page.items().first().cloned();
                                    logging::console_log(&format!("User found for email: {email}"));
                                }
                                _ => {
                                    logging::console_error(&format!(
                                        "Multiple users found for email: {email}"
                                    ));
                                }
                            }
                        }

                        logging::console_log(&format!("Retrieved user: {:#?}", &user));
                        Ok(user)
                    }
                    Err(e) => {
                        logging::console_log(&format!("Error querying Cosmos DB for users: {e}"));
                        Err(color_eyre::eyre::eyre!(
                            "Error querying Cosmos DB for books: {}",
                            e
                        ))
                    }
                }
            }
        }
    }
}
