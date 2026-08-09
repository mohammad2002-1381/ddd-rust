use domain::models::users::enums::user_role_type::UserRoleType;
use serde::Serialize;

#[derive(Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: String,
    pub refresh_token: String
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
    pub role: UserRoleType
}