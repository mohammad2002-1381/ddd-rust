use domain::models::users::repository::IUserRepository;
use serde::Deserialize;

use crate::{common::error::CommandError};

#[derive(Deserialize)]
pub struct UpdateUserCommand {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>
}

pub async fn handle<UserRepository: IUserRepository>(request: UpdateUserCommand, user_id: i32) -> Result<i32, CommandError> {
    let user_option = UserRepository::find_by_id(user_id).await?;
    match user_option {
        Some(mut user) => {
            user.set_first_name(request.first_name.unwrap_or(user.first_name().to_owned()));
            user.set_last_name(request.last_name.unwrap_or(user.last_name().to_owned()));
            user.set_email(request.email.unwrap_or(user.email().to_owned()));
            UserRepository::update(&mut user).await?;

            Ok(user_id)
        },
        None => return Err(CommandError::NotFound("user not found".to_string())),
    }
}