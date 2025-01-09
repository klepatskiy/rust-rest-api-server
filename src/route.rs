use crate::di_container::Container;
use crate::ui::http;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use axum::Router;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

pub fn create_router(container: Arc<Container>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let (app_router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(http::healthcheck_handler))
        .routes(routes!(http::user::create_user_handler))
        .split_for_parts();

    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api);

    app_router
        .with_state(container)
        .layer(cors)
        .merge(swagger_ui)
}
