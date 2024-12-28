use crate::domain::user::entity::User;
use async_trait::async_trait;
use shaku::Interface;
use sqlx::Error;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Interface {
    async fn create_user(&self, user: User) -> Result<(), Error>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<User, Error>;
    async fn get_users(&self) -> Result<Vec<User>, Error>;
}
