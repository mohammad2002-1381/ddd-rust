mod product_query_service;
mod user_query_service;

pub type ProductQueryService = product_query_service::PGProductQueryService;
pub type UserQueryService = user_query_service::PGUserQueryService;

#[macro_export]
macro_rules! base_query_service {
    ($service_struct:ty, $dto:ty, $entity:path, $id_type:ty) => {
        impl IBaseQueryService<$dto, $id_type> for $service_struct {
            async fn find_by_id(id: $id_type) -> Result<Option<$dto>, QueryError> {
                use sea_orm::EntityTrait;
                
                let model_opt = <$entity>::find_by_id(id)
                    .one(crate::db_context::get_db_connection())
                    .await?;

                Ok(model_opt.map(|m| m.map()))
            }
        }
    };
}