use read_model::{base_query_service::errors::QueryError, dto::product_dto::ProductDto, queries::product_query_service::IProductQueryService, query_request::get_product_by_id_query::GetProductByIdQuery};

pub async fn handle<ProductQueryService: IProductQueryService>(request: GetProductByIdQuery, user_id: i32) -> Result<ProductDto, QueryError> {
    let product_option = ProductQueryService::find_by_id_user_id(request, user_id).await?;
    match product_option {
        Some(product) => Ok(product),
        None => Err(QueryError::NotFound),
    }
}