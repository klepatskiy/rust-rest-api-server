use crate::di_container::Container;
use crate::ui::http;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn create_router(container: Arc<Container>) -> Router {
    Router::new()
        .route("/healthcheck", get(http::healthcheck_handler))
        .route("/user", post(http::user::create_user_handler))
        .with_state(container)
}
