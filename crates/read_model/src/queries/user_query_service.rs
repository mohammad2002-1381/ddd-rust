use crate::{base_query_service::{IBaseQueryService, errors::QueryError}, dto::user_dto::UserDto, query_request::get_user_paged_list_query::GetUserPagedListQuery, queryable::pagination::PaginatedResult};

pub trait IUserQueryService: IBaseQueryService<UserDto, i32> {
    fn paged_list(request: GetUserPagedListQuery) -> impl Future<Output = Result<PaginatedResult<UserDto>, QueryError>> + Send;
}