use read_model::{base_query_service::errors::QueryError, dto::user_dto::UserDto, queries::user_query_service::IUserQueryService, query_request::get_user_paged_list_query::GetUserPagedListQuery, queryable::pagination::PaginatedResult};

pub async fn handle<UserQueryService: IUserQueryService>(request: GetUserPagedListQuery) -> Result<PaginatedResult<UserDto>, QueryError> {
    let users = UserQueryService::paged_list(request).await?;
    Ok(users)
}