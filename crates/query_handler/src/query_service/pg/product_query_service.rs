use read_model::{base_query_service::{IBaseQueryService, errors::QueryError}, dto::product_dto::ProductDto, mapper::IMapper, queries::product_query_service::IProductQueryService, query_request::{get_product_by_id_query::GetProductByIdQuery, get_product_paged_list_query::GetProductPagedListQuery}, queryable::pagination::{PaginatedResult, PaginationParams}};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{base_query_service, db_context::get_db_connection, entity_configurations::product, queryable::pagination::IPaginationExtension};

#[derive(Clone)]
pub struct PGProductQueryService;

base_query_service!(
    PGProductQueryService, 
    ProductDto, 
    product::Entity, 
    i32
);

impl IProductQueryService for PGProductQueryService {
    async fn paged_list(request: GetProductPagedListQuery, user_id: i32) -> Result<PaginatedResult<ProductDto>, QueryError> {
        let result  = product::Entity::find()
            .filter(product::Column::UserId.eq(user_id))
            .paged_list(get_db_connection(), &PaginationParams::new(request.page_number, request.page_size))
            .await?
            .project_to();

        Ok(result)
    }
    
    async fn find_by_id_user_id(request: GetProductByIdQuery, user_id: i32) -> Result<Option<ProductDto>, QueryError> {
        let model_opt = product::Entity::find_by_id(request.id)
        .filter(product::Column::UserId.eq(user_id))
        .one(get_db_connection())
        .await?;

        let product_opt = match model_opt {
            Some(model) => Some(model.map()),
            None => None,
        };

        Ok(product_opt)
    }
}