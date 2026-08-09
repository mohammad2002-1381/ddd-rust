use domain::{base_repository::errors::RepositoryError, models::products::{product::Product, repository::IProductRepository}};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use crate::{base_command, base_query, db_context::get_db_connection, entity_configurations::product};

#[derive(Clone)]
pub struct PGProductRepository;

base_command!(
    PGProductRepository, 
    Product, 
    product::Entity,
    product::ActiveModel, 
    i32
);

base_query!(
    PGProductRepository, 
    Product, 
    product::Entity, 
    i32
);

impl IProductRepository for PGProductRepository {
    async fn find_by_id_user_id(id: i32, user_id: i32) -> Result<Option<Product>, RepositoryError> {
        let model_opt = product::Entity::find_by_id(id)
            .filter(product::Column::UserId.eq(user_id))
            .one(get_db_connection())
            .await?;

        let product_opt = match model_opt {
            Some(model) => Some(model.into()),
            None => None,
        };

        Ok(product_opt)
    }
}