use crate::{base_query_service::{IBaseQueryService, errors::QueryError}, dto::product_dto::ProductDto, query_request::{get_product_by_id_query::GetProductByIdQuery, get_product_paged_list_query::GetProductPagedListQuery}, queryable::pagination::PaginatedResult};

pub trait IProductQueryService: IBaseQueryService<ProductDto, i32> {
    fn paged_list(request: GetProductPagedListQuery, user_id: i32) -> impl Future<Output = Result<PaginatedResult<ProductDto>, QueryError>> + Send;
    fn find_by_id_user_id(request: GetProductByIdQuery, user_id: i32) -> impl Future<Output = Result<Option<ProductDto>, QueryError>> + Send;
}