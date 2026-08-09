use serde::{Deserialize, Serialize};

use crate::models::user::UserRoleType;

#[derive(Serialize, Deserialize)]
pub struct UserDto {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub role: UserRoleType,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String
}