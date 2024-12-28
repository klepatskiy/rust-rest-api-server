use shaku::module;
use crate::app::use_case::command::create_user::CreateUserCommandHandler;
use crate::app::use_case::query::user_by_id::UserQueryHandler;

use crate::repository::postgres::user::user_repository::{
    UserRepositoryImpl,
    UserRepositoryImplParameters
};

use crate::app::service::user::UserServiceImpl;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

module! {
    pub Container {
        components = [
            UserRepositoryImpl,
            UserServiceImpl,
            CreateUserCommandHandler,
            UserQueryHandler,
        ],
        providers = []
    }
}

impl Container {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Container::builder()
            .with_component_parameters::<UserRepositoryImpl>(UserRepositoryImplParameters{
                db: pool,
            })
            .build()
    }
}