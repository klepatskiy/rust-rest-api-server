use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct HealthCheckResponse {
    status: String,
    message: String,
}

#[utoipa::path(
    method(get),
    path = "/healthcheck",
    responses((status = 200, body = HealthCheckResponse))
)]
pub async fn healthcheck_handler() -> Json<HealthCheckResponse> {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    Json(HealthCheckResponse {
        status: "Success".to_string(),
        message: MESSAGE.to_string(),
    })
}
