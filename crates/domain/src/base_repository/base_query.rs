use crate::models::base_entity::IBaseEntity;
use super::errors::RepositoryError;

pub trait BaseQuery<T: IBaseEntity<Id>, Id> {
    fn find_by_id(id: Id) -> impl Future<Output = Result<Option<T>, RepositoryError>> + Send;
}