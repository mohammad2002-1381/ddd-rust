mod product_repository;
mod user_repository;

pub type ProductRepository = product_repository::PGProductRepository;
pub type UserRepository = user_repository::PGUserRepository;

#[macro_export]
macro_rules! base_command {
    ($repo_struct:ty, $model:ty, $entity:path, $active_model:path, $id_type:ty) => {
        impl domain::base_repository::base_command::BaseCommand<$model, $id_type> for $repo_struct {
            async fn _create_entity(entity: &$model) -> Result<$id_type, domain::base_repository::errors::RepositoryError> {
                use sea_orm::ActiveModelTrait; 
                
                let active_model: $active_model = entity.into();
                let inserted = active_model.insert(crate::db_context::get_db_connection()).await?;
                Ok(inserted.id)
            }

            async fn _update_entity(entity: &$model) -> Result<$id_type, domain::base_repository::errors::RepositoryError> {
                use sea_orm::ActiveModelTrait;
                
                let active_model: $active_model = entity.into();
                let updated = active_model.update(crate::db_context::get_db_connection()).await?;
                Ok(updated.id)
            }

            async fn delete(id: $id_type) -> Result<(), domain::base_repository::errors::RepositoryError> {
                use sea_orm::EntityTrait;
                
                <$entity>::delete_by_id(id)
                    .exec(crate::db_context::get_db_connection())
                    .await?;
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! base_query {
    ($repo_struct:ty, $model:ty, $entity:path, $id_type:ty) => {
        impl domain::base_repository::base_query::BaseQuery<$model, $id_type> for $repo_struct {
            async fn find_by_id(id: $id_type) -> Result<Option<$model>, domain::base_repository::errors::RepositoryError> {
                use sea_orm::EntityTrait;
                
                let model_opt = <$entity>::find_by_id(id)
                    .one(crate::db_context::get_db_connection())
                    .await?;
                
                Ok(model_opt.map(|m| m.into()))
            }
        }
    };
}