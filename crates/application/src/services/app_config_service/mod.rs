pub mod default_config_service;

// kernel/src/app_config.rs
pub trait IAppConfigService {
    fn base_url() -> &'static str;
}