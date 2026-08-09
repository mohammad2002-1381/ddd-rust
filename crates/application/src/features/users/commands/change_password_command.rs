use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};
use domain::models::users::repository::IUserRepository;
use serde::Deserialize;

use crate::{common::error::CommandError};

#[derive(Deserialize)]
pub struct ChangePasswordCommand {
    pub old_password: String,
    pub new_password: String
}

pub async fn handle<UserRepository: IUserRepository>(request: ChangePasswordCommand, user_id: i32) -> Result<(), CommandError> {
    let user_option = UserRepository::find_by_id(user_id).await?;
    match user_option {
        Some(mut user) => {
            let parsed_hash = PasswordHash::new(&user.password_hash()).unwrap();
            let argon2 = Argon2::default();
            match argon2.verify_password(request.old_password.as_bytes(), &parsed_hash) {
                Ok(_) => {
                    let salt = SaltString::generate(&mut OsRng);
                    let password_hash = argon2.hash_password(request.new_password.as_bytes(), &salt).unwrap().to_string();
                    user.set_password_hash(password_hash);
                    UserRepository::update(&mut user).await?;
                    Ok(())
                },
                Err(e) => return Err(CommandError::Unauthorized(format!("error on validating password: {}", e.to_string())))
            }
        },
        None => return Err(CommandError::NotFound("user not found".to_string())),
    }
}