use crate::base_repository::{base_command::BaseCommand, base_query::BaseQuery, errors::RepositoryError};
use super::product::Product;

pub trait IProductRepository: BaseCommand<Product, i32> + BaseQuery<Product, i32> {
    fn find_by_id_user_id(id: i32, user_id: i32) -> impl Future<Output = Result<Option<Product>, RepositoryError>> + Send;
}