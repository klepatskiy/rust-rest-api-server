mod route;
mod domain;
mod app;
mod repository;
mod ui;
mod di_container;

use std::sync::Arc;
use axum::http::{HeaderValue, Method};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use route::create_router;
use crate::di_container::Container;

pub struct ServerImpl {
    container: Container,
}

impl ServerImpl {
    pub fn new(container: Container) -> Self {
        ServerImpl { container }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await.expect("Failed to create pool");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let server = ServerImpl::new(Container::new(Arc::new(pool)));
    let app = create_router(Arc::new(server.container)).layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
