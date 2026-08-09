use domain::models::users::enums::user_role_type::UserRoleType;

pub mod default_jwt_service;

pub trait IJwtService {
    fn generate_jwt_token(user_id: i32, role: UserRoleType) -> String;
    fn generate_refresh_token() -> String;
    fn generate_api_token() -> String;
    fn get_secret<'a>() -> &'a str;
}