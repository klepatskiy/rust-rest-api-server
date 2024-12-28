use std::sync::Arc;
use async_trait::async_trait;
use shaku::Component;
use uuid::Uuid;
use crate::app::CommandHandler;
use crate::app::dto::user::CreateUserDto;
use crate::app::error::AppError;
use crate::app::error::service_error::ServiceError;
use crate::domain::user::service::UserService;

pub struct CreateUserCommand {
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Component)]
#[shaku(interface = CommandHandler<CreateUserCommand>)]
pub struct CreateUserCommandHandler {
    #[shaku(inject)]
    user_service: Arc<dyn UserService>,
}

#[async_trait]
impl CommandHandler<CreateUserCommand> for CreateUserCommandHandler {
    async fn handle(&self, command: CreateUserCommand) -> Result<Uuid, AppError> {
        let operation_id = Uuid::now_v7();

        match self.user_service.create_user(CreateUserDto{
            id: Uuid::now_v7(),
            first_name: command.first_name,
            last_name: command.last_name,
        }).await {
            Ok(_) => Ok(operation_id),
            Err(e) => match e {
                ServiceError::NotFoundError => Err(AppError::NotFoundError),
                _ => Err(AppError::SomeError)
            }
        }
    }
}