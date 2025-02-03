use uuid::Uuid;

pub struct CreateUserDto {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: Option<String>,
}
