use application::{
    features::users::{
        commands::{
            delete_user_by_id_command::{self, DeleteUserByIdCommand}, 
            login_into_user_command::{self, LoginIntoUserCommand}, 
            update_user_by_id_command::{self, UpdateUserByIdCommand}
        }, 
        queries::get_users_list_query
    }, 
    services::JwtService
};
use axum::{
    Router, 
    extract::{Path, Query}, 
    middleware::from_fn_with_state, 
    response::IntoResponse, 
    routing::{get, post}
};
use domain::models::users::enums::user_role_type::UserRoleType;
use infrastructure::repositories::UserRepository;
use query_handler::query_services::UserQueryService;
use read_model::query_request::get_user_paged_list_query::GetUserPagedListQuery;

use crate::{
    middlewares::{
        authorization::{auth, require_roles}, 
        error_exception::ApiError,
        param_validation::JsonParam,
        response::ApiResponse
    }, 
    state::SharedState
};

pub trait IUsersController {
    fn users_router(self) -> Router<SharedState>;
}

impl IUsersController for Router<SharedState> {
    fn users_router(self) -> Router<SharedState> {
        let protected_routes = Router::new()
            // Chain HTTP methods for the same path to prevent Axum panics
            .route(
                "/", 
                get(get_users_list)
                    .patch(update_user_by_id)
                    .delete(delete_user_by_id)
            )
            .route("/login/{id}", post(login_into_user))
            .route_layer(from_fn_with_state(vec![UserRoleType::Admin], require_roles))
            .layer(axum::middleware::from_fn(auth));

        protected_routes
    }
}

async fn get_users_list(
    Query(query): Query<GetUserPagedListQuery>
) -> Result<impl IntoResponse, ApiError> {
    let result = get_users_list_query::handle::<UserQueryService>(query).await?;
    Ok(ApiResponse::ok(result))
}

async fn update_user_by_id(
    JsonParam(command): JsonParam<UpdateUserByIdCommand>
) -> Result<impl IntoResponse, ApiError> {
    let result = update_user_by_id_command::handle::<UserRepository>(command).await?;
    Ok(ApiResponse::ok(result))
}

async fn delete_user_by_id(
    JsonParam(command): JsonParam<DeleteUserByIdCommand>
) -> Result<impl IntoResponse, ApiError> {
    let _ = delete_user_by_id_command::handle::<UserRepository>(command).await?;
    
    // A 204 No Content response should not include a body
    Ok(ApiResponse::no_content())
}

async fn login_into_user(
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    let command = LoginIntoUserCommand::new(id);
    let result = login_into_user_command::handle::<UserRepository, JwtService>(command).await?;
    
    Ok(ApiResponse::created(result))
}
