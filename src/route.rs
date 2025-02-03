use crate::app::dto::user::user_dto::UserDto;
use crate::app::use_case::bus::{CommandHandler, QueryHandler};
use crate::app::use_case::command::create_user::create_user_command::CreateUserCommand;
use crate::app::use_case::query::user_by_id::UserQuery;
use crate::di_container::Container;
use crate::ui::http;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use axum::Router;
use shaku::HasComponent;
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

    let create_user_command: Arc<dyn CommandHandler<CreateUserCommand>> = container.resolve();
    let user_query: Arc<dyn QueryHandler<UserQuery, UserDto>> = container.resolve();

    let (app_router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(http::healthcheck::healthcheck_handler))
        .routes(routes!(http::user::create_user::create_user_handler))
        .with_state(create_user_command)
        .routes(routes!(http::user::get_user::get_user_handler))
        .with_state(user_query)
        .split_for_parts();

    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api);

    app_router.layer(cors).merge(swagger_ui)
}
