use crate::app::use_case::bus::CommandHandler;
use crate::app::use_case::command::create_user::create_user_command::CreateUserCommand;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_valid::Valid;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use validator::Validate;

#[derive(ToSchema, Deserialize, Validate, Clone, Debug)]
pub struct UserCreate {
    #[serde(rename = "first_name")]
    #[validate(length(min = 3, message = "First name must be at least 3 characters long"))]
    #[schema(example = "Robert", min_length = 3)]
    pub first_name: String,

    #[serde(rename = "last_name")]
    #[schema(example = "Star")]
    pub last_name: Option<Option<String>>,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub struct UserCreateSuccessResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "operationId")]
    pub operation_id: String,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub struct UserCreateErrorResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "message")]
    pub message: String,
}

#[utoipa::path(
    method(post),
    path = "/user",
    request_body = UserCreate,
    responses(
        (status = 201, description = "Created", body = UserCreateSuccessResponse),
        (status = 400, description = "Validation error", body = UserCreateErrorResponse)
    ),
)]
pub async fn create_user_handler(
    State(handler): State<Arc<dyn CommandHandler<CreateUserCommand>>>,
    user: Valid<Json<UserCreate>>,
) -> Result<Json<UserCreateSuccessResponse>, (StatusCode, Json<UserCreateErrorResponse>)> {
    const MESSAGE: &str = "Success";
    const STATUS: &str = "User created";
    let command = CreateUserCommand {
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone().flatten(),
    };

    let result = handler.handle(command).await;
    match result {
        Ok(operation_id) => Ok(Json(UserCreateSuccessResponse {
            status: STATUS.to_string(),
            message: MESSAGE.to_string(),
            operation_id: operation_id.to_string(),
        })),
        Err(_) => Err((
            StatusCode::BAD_REQUEST,
            Json(UserCreateErrorResponse {
                status: "Error".to_string(),
                message: "Failed to create user".to_string(),
            }),
        )),
    }
}
