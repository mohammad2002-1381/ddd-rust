use super::user::User;
use crate::base_repository::{base_command::BaseCommand, base_query::BaseQuery, errors::RepositoryError};

pub trait IUserRepository: BaseQuery<User, i32> + BaseCommand<User, i32> {
    fn find_by_email_async(email: &str) -> impl Future<Output = Result<Option<super::user::User>, RepositoryError>> + Send;
    fn create_user(entity: &User) -> impl Future<Output = Result<User, RepositoryError>> + Send;
    fn create_token(token: &super::token::Token) -> impl Future<Output = Result<super::token::Token, RepositoryError>> + Send;
}
