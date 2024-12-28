use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use shaku::HasComponent;
use crate::app::CommandHandler;
use crate::app::use_case::command::create_user::{CreateUserCommand};
use crate::di_container::Container;
use openapi::models::{UserCreate, UserCreateSuccessResponse};


pub async fn create_user_handler(
    State(container): State<Arc<Container>>,
    user: Json<UserCreate>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
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
            let response_body = UserCreateSuccessResponse {
                status: STATUS.to_string(),
                message: MESSAGE.to_string(),
                operation_id,
            };

            Ok((StatusCode::CREATED, Json(response_body)))
        }
        Err(e) => {
            let json_error = serde_json::json!({
                "status": "error",
                "message": format!("Something happened: {e}")
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json_error)))
        }
    }
}
