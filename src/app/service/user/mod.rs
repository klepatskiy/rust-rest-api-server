use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use shaku::Component;
use uuid::Uuid;
use crate::app::dto::user::{CreateUserDto, UserDto};
use crate::app::error::service_error::ServiceError;
use crate::domain::user::entity::User;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::service::UserService;

#[derive(Component)]
#[shaku(interface = UserService)]
pub struct UserServiceImpl {
    #[shaku(inject)]
    user_repository: Arc<dyn UserRepository>,
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(&self, create_user: CreateUserDto) -> Result<(), ServiceError> {
        let user = User {
            id: create_user.id,
            first_name: create_user.first_name,
            last_name: create_user.last_name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        match self.user_repository.create_user(user).await {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?}", e);
                Err(ServiceError::ServiceInternalError)
            },
        }
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<UserDto, ServiceError> {
        match self.user_repository.get_user_by_id(user_id).await {
            Ok(user) => Ok(UserDto {
                id: user.id,
                first_name: user.first_name,
                last_name: user.last_name,
                created_at: user.created_at,
                updated_at: user.updated_at,
            }),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Err(ServiceError::NotFoundError),
                _ => Err(ServiceError::ServiceInternalError),
            },
        }
    }

    async fn get_users(&self, _: Uuid) -> Result<Vec<UserDto>, ServiceError> {
        todo!()
    }
}
