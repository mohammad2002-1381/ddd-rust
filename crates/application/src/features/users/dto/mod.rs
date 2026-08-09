use domain::models::{base_entity::IBaseEntity, users::user::User};

pub mod auth_dto;

impl From<User> for auth_dto::UserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id().unwrap_or_default(),
            first_name: value.first_name().to_owned(),
            last_name: value.last_name().to_owned(),
            email: value.email().to_owned(),
            created_at: value.created_at().to_rfc3339(),
            updated_at: value.updated_at().to_rfc3339(),
            role: value.role(),
        }
    }
}