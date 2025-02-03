use crate::app::dto::user::create_user_dto::CreateUserDto;
use crate::app::dto::user::user_dto::UserDto;
use crate::app::error::service_error::ServiceError;
use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;

#[async_trait]
pub trait UserService: Interface {
    async fn create_user(&self, create_user: CreateUserDto) -> Result<(), ServiceError>;
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<UserDto, ServiceError>;
    async fn get_users(&self, user_id: Uuid) -> Result<Vec<UserDto>, ServiceError>;
}
