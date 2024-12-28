use crate::app::use_case::command::create_user::CreateUserCommand;
use crate::app::CommandHandler;
use crate::di_container::Container;
use axum::extract::State;
use axum::Json;
use shaku::HasComponent;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCreate {
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: Option<Option<String>>,
}


#[derive(ToSchema, Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCreateSuccessResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "operationId")]
    pub operation_id: String,
}

#[utoipa::path(
    method(post),
    path = "/user",
    request_body = UserCreate,
   responses(
        (status = 201, description = "Created"),
    ),
)]
pub async fn create_user_handler(
    State(container): State<Arc<Container>>,
    user: Json<UserCreate>,
) -> Json<UserCreateSuccessResponse> {
    const MESSAGE: &str = "Success";
    const STATUS: &str = "User created";
    let command = CreateUserCommand {
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone().flatten(),
    };

    let handler: &dyn CommandHandler<CreateUserCommand> = container.resolve_ref();
    let result = handler.handle(command).await;

    match result {
        Ok(operation_id) => {
             Json(UserCreateSuccessResponse {
                status: STATUS.to_string(),
                message: MESSAGE.to_string(),
                operation_id: operation_id.to_string(),
            })
        }
        Err(_) => {
            Json(UserCreateSuccessResponse {
                status: "Error".to_string(),
                message: "Error".to_string(),
                // operation_id: uuid::Uuid::nil(),
                operation_id: "dsds".to_string(),
            })
        }
    }
}
