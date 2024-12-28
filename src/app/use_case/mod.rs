pub mod command;
pub mod query;
pub mod error;
pub mod dto;
mod service;

use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;
use crate::app::error::AppError;

#[async_trait]
pub trait CommandHandler<C>: Interface + Send + Sync {
    async fn handle(&self, command: C) -> Result<Uuid, AppError>;
}

#[async_trait]
pub trait QueryHandler<Q, R>: Interface + Send + Sync {
    async fn handle(&self, query: Q) -> Result<R, AppError>;
}
