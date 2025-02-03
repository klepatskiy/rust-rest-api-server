use crate::app::use_case::bus::QueryHandler;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use crate::app::dto::user::user_dto::UserDto;
use crate::app::use_case::query::user_by_id::UserQuery;

#[derive(ToSchema, Serialize, Deserialize)]
pub struct UserResponse {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: i64,
    #[serde(rename = "updatedAt")]
    updated_at: i64,
}


#[utoipa::path(
    method(get),
    path = "/user/{id}",
    responses(
        (status = 200, description = "user", body = UserResponse),
        (status = 404, description = "Not Found"),
    ),
)]
pub async fn get_user_handler(
    Path(id): Path<String>,
    State(handler): State<Arc<dyn QueryHandler<UserQuery, UserDto>>>,
) -> Result<Json<UserResponse>, StatusCode> {
    let command = UserQuery {
        uuid: id,
    };

    let result = handler.handle(command).await;
    match result {
        Ok(user_dto) => Ok(Json(UserResponse {
            id: user_dto.id.to_string(),
            first_name: user_dto.first_name,
            last_name: user_dto.last_name,
            created_at: user_dto.created_at.timestamp(),
            updated_at: user_dto.updated_at.timestamp(),
        })),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
