use crate::aggregate_root::IAggregateRoot;
use super::errors::RepositoryError;

pub trait BaseQuery<T: IAggregateRoot<Id>, Id> {
    fn find_by_id(id: Id) -> impl Future<Output = Result<Option<T>, RepositoryError>> + Send;
}