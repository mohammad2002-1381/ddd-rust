use argon2::{Argon2, PasswordHash, PasswordVerifier};
use domain::models::{base_entity::IBaseEntity, users::{repository::IUserRepository, token::Token}};
use serde::Deserialize;

use crate::{common::error::CommandError, features::users::dto::auth_dto::AuthResponse, services::IJwtService};

#[derive(Deserialize)]
pub struct LoginUserCommand {
    pub email: String,
    pub password: String
}

pub async fn handle<UserRepository: IUserRepository, JwtService: IJwtService>(request: LoginUserCommand) -> Result<AuthResponse, CommandError> {
    let user_option = UserRepository::find_by_email_async(&request.email).await?;
    match user_option {
        Some(user) => {
            let parsed_hash = PasswordHash::new(user.password_hash())
                .map_err(|e| CommandError::Internal(format!("Invalid hash format in DB: {}", e)))?;
            let argon2 = Argon2::default();
            match argon2.verify_password(request.password.as_bytes(), &parsed_hash) {
                Ok(_) => {
                    let token = JwtService::generate_jwt_token(user.id().unwrap_or_default(), user.role());
                    let refresh_token = JwtService::generate_refresh_token();
                    match UserRepository::create_token(&Token::new(token, refresh_token, user.id().unwrap_or_default())).await {
                        Ok(token) => {
                            return Ok(AuthResponse {
                                user: user.into(),
                                token: token.token().to_owned(),
                                refresh_token: token.refresh_token().to_owned()
                            });
                        },
                        Err(e) => {
                            return Err(CommandError::Internal(e.to_string()));
                        },
                    }
                },
                Err(e) => return Err(CommandError::Unauthorized(format!("error on validating password: {}", e.to_string())))
            }
            
        },
        None => Err(CommandError::NotFound("user not found".to_string())),
    }
}