use read_model::{base_query_service::errors::QueryError, dto::product_dto::ProductDto, queries::product_query_service::IProductQueryService, query_request::get_product_paged_list_query::GetProductPagedListQuery, queryable::pagination::PaginatedResult};

pub async fn handle<ProductQueryService: IProductQueryService>(request: GetProductPagedListQuery, user_id: i32) -> Result<PaginatedResult<ProductDto>, QueryError> {
    let all = ProductQueryService::paged_list(request, user_id).await?;
    return Ok(all);
}