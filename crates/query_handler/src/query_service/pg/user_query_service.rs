use read_model::{base_query_service::{IBaseQueryService, errors::QueryError}, dto::user_dto::UserDto, mapper::IMapper, queries::user_query_service::IUserQueryService, query_request::get_user_paged_list_query::GetUserPagedListQuery, queryable::pagination::{PaginatedResult, PaginationParams}};
use sea_orm::EntityTrait;

use crate::{base_query_service, db_context::get_db_connection, entity_configurations::user, queryable::pagination::IPaginationExtension};

#[derive(Clone)]
pub struct PGUserQueryService;

base_query_service!(
    PGUserQueryService, 
    UserDto, 
    user::Entity, 
    i32
);

impl IUserQueryService for PGUserQueryService{
    async fn paged_list(request: GetUserPagedListQuery) -> Result<PaginatedResult<UserDto>, QueryError> {
        let result  = user::Entity::find()
            .paged_list(get_db_connection(), &PaginationParams::new(request.page_number, request.page_size))
            .await?
            .project_to();

        Ok(result)
    }
}