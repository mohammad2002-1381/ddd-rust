use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
use domain::models::{base_entity::IBaseEntity, users::{enums::user_role_type::UserRoleType, repository::IUserRepository, token::Token, user::User}};
use serde::Deserialize;

use crate::{common::error::CommandError, features::users::dto::auth_dto::AuthResponse, services::IJwtService};


#[derive(Deserialize)]
pub struct RegisterUserCommand {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub is_active: bool,
}

pub async fn handle<UserRepository: IUserRepository, JwtService: IJwtService>(request: RegisterUserCommand) -> Result<AuthResponse, CommandError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash_password = argon2.hash_password(request.password.as_bytes(), &salt).unwrap().to_string();
    let user = UserRepository::create_user(&User::new(
        &request.first_name,
        &request.last_name,
        &request.email,
        &hash_password,
        UserRoleType::User,
        request.is_active)).await?;
    let token = JwtService::generate_jwt_token(user.id().unwrap_or_default(), user.role());
    let refresh_token = JwtService::generate_refresh_token();
    let token = UserRepository::create_token(&Token::new(token, refresh_token, user.id().unwrap_or_default())).await?;
    return Ok(AuthResponse {
        user: user.into(),
        token: token.token().to_owned(),
        refresh_token: token.refresh_token().to_owned()
    });
}
