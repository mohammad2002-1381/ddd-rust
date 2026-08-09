use std::sync::LazyLock;

static BASE_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("PUBLIC_BASE_URL").expect("PUBLIC_BASE_URL must be set")
});

pub struct DefaultConfigService;

impl super::IAppConfigService for DefaultConfigService {
    fn base_url() -> &'static str { &BASE_URL }
}