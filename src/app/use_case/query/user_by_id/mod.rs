use std::str::FromStr;
use crate::app::dto::user::{UserDto};
use crate::app::error::service_error::ServiceError;
use crate::app::error::AppError;
use crate::app::QueryHandler;
use crate::domain::user::service::UserService;
use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserQuery {
    pub uuid: String,
}

#[derive(Component)]
#[shaku(interface = QueryHandler<UserQuery, UserDto>)]
pub struct UserQueryHandler {
    #[shaku(inject)]
    user_service: Arc<dyn UserService>,
}

#[async_trait]
impl QueryHandler<UserQuery, UserDto> for UserQueryHandler {
    async fn handle(&self, query: UserQuery) -> Result<UserDto, AppError> {
        let uuid = Uuid::from_str(&*query.uuid).map_err(|_| AppError::SomeError)?;

        match self.user_service.get_user_by_id(uuid).await {
            Ok(user) => Ok(user),
            Err(e) => match e {
                ServiceError::NotFoundError => Err(AppError::NotFoundError),
                _ => Err(AppError::SomeError),
            },
        }
    }
}
