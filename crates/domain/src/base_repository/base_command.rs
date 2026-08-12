use crate::aggregate_root::IAggregateRoot;
use super::errors::RepositoryError;

pub trait BaseCommand<T: IAggregateRoot<Id>, Id> {
    fn _create_entity(entity: &T) -> impl Future<Output = Result<Id, RepositoryError>> + Send;
    fn _update_entity(entity: &T) -> impl Future<Output = Result<Id, RepositoryError>> + Send;
    fn delete(id: Id) -> impl Future<Output = Result<(), RepositoryError>> + Send;

    fn create(entity: &T) -> impl Future<Output = Result<Id, RepositoryError>> + Send {
        Self::_create_entity(entity)
    }

    fn update(entity: &mut T) -> impl Future<Output = Result<Id, RepositoryError>> + Send {
        entity.touch();

        Self::_update_entity(entity)
    }
}