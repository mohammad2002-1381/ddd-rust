use domain::{base_repository::errors::RepositoryError, models::users::{repository::IUserRepository, token::Token, user::User}};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::{base_command, base_query, db_context::get_db_connection, entity_configurations::{token, user}};

#[derive(Clone)]
pub struct PGUserRepository;

base_command!(
    PGUserRepository, 
    User, 
    user::Entity,
    user::ActiveModel, 
    i32
);

base_query!(
    PGUserRepository, 
    User, 
    user::Entity, 
    i32
);

impl IUserRepository for PGUserRepository {
    async fn find_by_email_async(email: &str) -> Result<Option<User>, RepositoryError> {
        let model_opt = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(get_db_connection())
        .await?;

        let product_opt = match model_opt {
            Some(model) => Some(model.into()),
            None => None,
        };

        Ok(product_opt)
    }
    
    async fn create_token(token: &Token) -> Result<Token, RepositoryError> {
        let active_model: token::ActiveModel = token.into();

        let inserted_model = active_model
            .insert(get_db_connection())
            .await?;
        
        Ok(inserted_model.into())
    }
    
    async fn create_user(entity: &User) -> Result<User, RepositoryError> {
        let active_model: user::ActiveModel = entity.into();

        let inserted_model = active_model
            .insert(get_db_connection())
            .await?;
        
        Ok(inserted_model.into())
    }
}