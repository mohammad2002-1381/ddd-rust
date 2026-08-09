mod cache_service;
mod jwt_service;
mod app_config_service;
mod current_user_service;

pub type CacheService = cache_service::redis_cache_service::RedisCacheService;
pub type JwtService = jwt_service::default_jwt_service::DefaultJwtService;
pub type AppConfigService = app_config_service::default_config_service::DefaultConfigService;
pub type Claims = jwt_service::default_jwt_service::Claims;

pub use jwt_service::IJwtService;
pub use cache_service::ICacheService;
pub use app_config_service::IAppConfigService;
pub use current_user_service::{current_user_request::CurrentUserRequest, current_user_service::CurrentUserService, ICurrentUserService};

#[derive(Debug)]
pub enum Error {
    MissingEnvVar(String),
    ConnectionFailed(String),
    AlreadyInitialized(String),
}

pub trait ServiceRegister {
    fn register_service() -> impl Future<Output = Result<(), Error>> + Send;
}

pub async fn init_services() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    CacheService::register_service().await?;

    Ok(())
}