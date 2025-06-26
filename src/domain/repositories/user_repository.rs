// Repository traits (interfaces)

use crate::domain::models::user::User;
use async_trait::async_trait;
use color_eyre::Result;

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: User) -> Result<()>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>>;
}
