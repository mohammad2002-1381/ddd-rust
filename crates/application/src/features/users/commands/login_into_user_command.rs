use domain::models::{base_entity::IBaseEntity, users::repository::IUserRepository};
use serde::Deserialize;

use crate::{common::error::CommandError, features::users::dto::auth_dto::AuthResponse, services};

#[derive(Deserialize)]
pub struct LoginIntoUserCommand {
    pub id: i32
}

impl LoginIntoUserCommand {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

pub async fn handle<UserRepository: IUserRepository, JwtService: services::IJwtService>(request: LoginIntoUserCommand) -> Result<AuthResponse, CommandError> {
    let user_option = UserRepository::find_by_id(request.id).await?;
    match user_option {
        Some(user) => {
            let token = JwtService::generate_jwt_token(user.id().unwrap_or_default(), user.role());
            let refresh_token = JwtService::generate_refresh_token();
            return Ok(AuthResponse {
                user: user.into(),
                token,
                refresh_token
            });
        },
        None => Err(CommandError::NotFound("user not found".to_string())),
    }
}