use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct CreateUserDto {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: Option<String>,
}

pub struct UserDto {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
