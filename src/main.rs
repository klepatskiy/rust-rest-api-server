mod app;
mod di_container;
mod domain;
mod repository;
mod route;
mod ui;

use crate::di_container::Container;
use dotenv::dotenv;
use route::create_router;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

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
        .await
        .expect("Failed to create pool");

    let server = ServerImpl::new(Container::new(Arc::new(pool)));
    let router = create_router(Arc::new(server.container));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
