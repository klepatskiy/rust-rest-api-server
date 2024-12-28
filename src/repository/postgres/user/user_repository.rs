use crate::domain::user::entity::User;
use crate::domain::user::repository::UserRepository;
use async_trait::async_trait;
use shaku::Component;
use sqlx::{Error, Pool, Postgres};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct UserRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(&self, user: User) -> Result<(), Error> {
        let sql = r#"
            INSERT INTO users (id, first_name, last_name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
        "#;

        sqlx::query(sql)
            .bind(user.id)
            .bind(&user.first_name)
            .bind(&user.last_name)
            .bind(user.created_at)
            .bind(user.updated_at)
            .execute(&*self.db)
            .await?;

        Ok(())
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<User, Error> {
        let sql = r#"
            SELECT id, first_name, last_name, nickname, created_at, updated_at
            FROM users
            WHERE id = $1
        "#;

        sqlx::query_as::<_, User>(sql)
            .bind(id)
            .fetch_one(&*self.db)
            .await
    }

    async fn get_users(&self) -> Result<Vec<User>, Error> {
        todo!()
    }
}
