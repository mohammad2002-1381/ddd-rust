use application::{
    features::users::{
        commands::{
            change_password_command::{self, ChangePasswordCommand}, 
            login_user_command::{self, LoginUserCommand}, 
            logout_user_command::{self, LogoutUserCommand}, 
            register_user_command::{self, RegisterUserCommand}, 
            update_user_command::{self, UpdateUserCommand}
        }, 
        queries::get_user_query
    }, 
    services::{CurrentUserRequest, ICurrentUserService, JwtService}
};
use axum::{
    Router, 
    http::HeaderMap, 
    middleware::from_fn, 
    response::IntoResponse, 
    routing::{get, post}
};
use infrastructure::repositories::UserRepository;
use query_handler::query_services::UserQueryService;
use crate::{
    middlewares::{
        authorization::auth, 
        error_exception::ApiError, 
        param_validation::JsonParam, 
        response::ApiResponse
    }, 
    state::SharedState
};

pub trait IUserController {
    fn user_router(self) -> Router<SharedState>;
}

impl IUserController for Router<SharedState> {
    fn user_router(self) -> Router<SharedState> {
        let public_routes = Router::new()
            .route("/register", post(create_user))
            .route("/login", post(login_user));

        let protected_routes = Router::new()
            .route("/", get(get_profile))
            .route("/logout", post(logout_user))
            // .route("/refresh_token", post(refresh_token))
            .route("/change_password", post(change_password))
            .route("/update", post(update_user))
            .route("/id", get(get_user_id))
            .route_layer(from_fn(auth));

        self
            .merge(public_routes)
            .merge(protected_routes)
    }
}

async fn create_user(
    JsonParam(command): JsonParam<RegisterUserCommand>
) -> Result<impl IntoResponse, ApiError> {
    let result = register_user_command::handle::<UserRepository, JwtService>(command).await?;
    Ok(ApiResponse::created(result))
}

async fn login_user(
    JsonParam(command): JsonParam<LoginUserCommand>
) -> Result<impl IntoResponse, ApiError> {
    let result = login_user_command::handle::<UserRepository, JwtService>(command).await?;
    Ok(ApiResponse::ok(result))
}

async fn logout_user(
    headers: HeaderMap
) -> Result<impl IntoResponse, ApiError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer ")) // Strips "Bearer "
        .map(|s| s.to_string())
        .unwrap_or_default();

    if token.is_empty() {
        return Ok(ApiResponse::no_content());
    }

    let command = LogoutUserCommand::new(token);
    let _ = logout_user_command::handle(command).await?;

    Ok(ApiResponse::no_content())
}

async fn change_password (
    CurrentUserRequest(current_user): CurrentUserRequest,
    JsonParam(command): JsonParam<ChangePasswordCommand>
) -> Result<impl IntoResponse, ApiError> {
    let result = change_password_command::handle::<UserRepository>(command, current_user.user_id()?).await?;
    Ok(ApiResponse::ok(result))
}

async fn update_user (
    CurrentUserRequest(current_user): CurrentUserRequest,
    JsonParam(command): JsonParam<UpdateUserCommand>
 ) -> Result<impl IntoResponse, ApiError> {
    let result = update_user_command::handle::<UserRepository>(command, current_user.user_id()?).await?;
    Ok(ApiResponse::ok(result))
 }

// async fn refresh_token(
//     CurrentUserRequest(current_user): CurrentUserRequest,
//     JsonParam(command): JsonParam<RefreshTokenCommand>,
// ) -> Result<impl IntoResponse, ApiError> {
//     let handler = RefreshTokenCommandHandler::new(&current_user);
//     let result = handler.handle(command).await?;
//     Ok(ApiResponse::created(result))
// }

async fn get_profile(
    // State(app_state): State<SharedState>,
    // Query(query): Query<GetUserByIdQuery>,
    CurrentUserRequest(current_user): CurrentUserRequest
) -> Result<impl IntoResponse, ApiError> {
    let result = get_user_query::handle::<UserQueryService>(current_user.user_id()?).await?;
    Ok(ApiResponse::ok(result))
}

async fn get_user_id(
    CurrentUserRequest(current_user): CurrentUserRequest
) -> Result<impl IntoResponse, ApiError> {
    let user_id = current_user.user_id()?;
    Ok(ApiResponse::ok(user_id))
}