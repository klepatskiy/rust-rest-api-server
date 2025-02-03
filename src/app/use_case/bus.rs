use crate::app::error::AppError;
use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;

#[async_trait]
pub trait CommandHandler<C>: Interface + Send + Sync {
    async fn handle(&self, command: C) -> Result<Uuid, AppError>;
}

#[async_trait]
pub trait QueryHandler<Q, R>: Interface + Send + Sync {
    async fn handle(&self, query: Q) -> Result<R, AppError>;
}
