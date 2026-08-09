use domain::models::{base_entity::IBaseEntity, users::repository::IUserRepository};
use serde::Deserialize;

use crate::common::error::CommandError;

#[derive(Deserialize)]
pub struct UpdateUserByIdCommand {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>
}

pub async fn handle<UserRepository: IUserRepository>(request: UpdateUserByIdCommand) -> Result<i32, CommandError> {
    let user_option = UserRepository::find_by_id(request.id).await?;
    
    match user_option {
        Some(mut user) => {
            if let Some(first_name) = request.first_name {
                user.set_first_name(first_name)
            }
            if let Some(last_name) = request.last_name {
                user.set_first_name(last_name);
            }
            if let Some(email) = request.email {
                user.set_email(email);
            }
            _ = UserRepository::update(&mut user).await?;
            Ok(user.id().unwrap_or_default())
        },
        None => Err(CommandError::NotFound("user not found".to_string())),
    }
}