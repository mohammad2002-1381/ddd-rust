use application::{features::products::{
    commands::{
        create_product_command::{self, CreateProductCommand},
        delete_product_command::{self, DeleteProductCommand},
        update_product_command::{self, UpdateProductCommand},
    },
    queries::{get_product_by_id_query, get_products_list_query},
}, services::{CurrentUserRequest, ICurrentUserService}};
use axum::{
    Router,
    extract::{Path, Query},
    middleware::from_fn,
    routing::{delete, get, patch, post},
};
use infrastructure::repositories::ProductRepository;
use query_handler::query_services::ProductQueryService;
use read_model::{
    dto::product_dto::ProductDto,
    query_request::{get_product_by_id_query::GetProductByIdQuery, get_product_paged_list_query::GetProductPagedListQuery},
    queryable::pagination::PaginatedResult,
};

use crate::{
    middlewares::{authorization::auth, error_exception::ApiError, param_validation::JsonParam, response::ApiResponse}, state::SharedState,
};

pub trait IProductController {
    fn product_router(self) -> Router<SharedState>;
}

impl IProductController for Router<SharedState> {
    fn product_router(self) -> Router<SharedState> {
        self
            .route("/", post(create_product))
            .route("/", patch(update_product))
            .route("/", delete(delete_product))
            .route("/", get(get_product_list))
            .route("/{id}", get(get_product_by_id))
            .route_layer(from_fn(auth))
    }
}

pub fn product_router() -> Router<SharedState> {
    Router::new()
        .route("/", post(create_product))
        .route("/", patch(update_product))
        .route("/", delete(delete_product))
        .route("/", get(get_product_list))
        .route("/{id}", get(get_product_by_id))
        .route_layer(from_fn(auth))
}

async fn create_product(
    CurrentUserRequest(user): CurrentUserRequest,
    JsonParam(cmd): JsonParam<CreateProductCommand>,
) -> Result<ApiResponse<i32>, ApiError> {
    Ok(ApiResponse::created(create_product_command::handle::<ProductRepository>(cmd, user.user_id()?)
        .await?))
}

async fn update_product(
    CurrentUserRequest(user): CurrentUserRequest,
    JsonParam(cmd): JsonParam<UpdateProductCommand>,
) -> Result<ApiResponse<i32>, ApiError> {
    Ok(ApiResponse::ok(update_product_command::handle::<ProductRepository>(cmd, user.user_id()?)
        .await?))
}

async fn delete_product(
    CurrentUserRequest(user): CurrentUserRequest,
    JsonParam(cmd): JsonParam<DeleteProductCommand>,
) -> Result<ApiResponse<()>, ApiError> {
    delete_product_command::handle::<ProductRepository>(cmd, user.user_id()?).await?;
    Ok(ApiResponse::no_content())
}

async fn get_product_by_id(
    Path(id): Path<i32>,
    CurrentUserRequest(user): CurrentUserRequest,
) -> Result<ApiResponse<ProductDto>, ApiError> {
    Ok(ApiResponse::ok(get_product_by_id_query::handle::<ProductQueryService>(GetProductByIdQuery::new(id), user.user_id()?)
        .await?))
}

async fn get_product_list(
    Query(query): Query<GetProductPagedListQuery>,
    CurrentUserRequest(user): CurrentUserRequest,
) -> Result<ApiResponse<PaginatedResult<ProductDto>>, ApiError> { 
    Ok(ApiResponse::ok(get_products_list_query::handle::<ProductQueryService>(query, user.user_id()?)
        .await?))
}