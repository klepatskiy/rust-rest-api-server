pub mod dto;
pub mod error;
pub mod service;
pub mod use_case;

use crate::app::error::AppError;
use crate::app::use_case::command::create_user::CreateUserCommand;
use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;

#[async_trait]
pub trait CommandHandler<C>: Interface + Send + Sync {
    async fn handle(&self, command: CreateUserCommand) -> Result<Uuid, AppError>;
}

#[async_trait]
pub trait QueryHandler<Q, R>: Interface + Send + Sync {
    async fn handle(&self, query: Q) -> Result<R, AppError>;
}
