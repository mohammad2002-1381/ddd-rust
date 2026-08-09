use read_model::{base_query_service::errors::QueryError, dto::user_dto::UserDto, queries::user_query_service::IUserQueryService};

pub async fn handle<UserQueryService: IUserQueryService>(user_id: i32) -> Result<UserDto, QueryError> {
    let user_option = UserQueryService::find_by_id(user_id).await?;
    match user_option {
        Some(user) => Ok(user),
        None => return Err(QueryError::NotFound),
    }
}