use std::{sync::LazyLock, time::{SystemTime, UNIX_EPOCH}};
use domain::models::users::enums::user_role_type::UserRoleType;
use jsonwebtoken::{EncodingKey, Header, encode};
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims { exp: usize, pub sub: String, pub role: UserRoleType }

struct JwtConfig { exp_seconds: usize, secret: String }

static CONFIG: LazyLock<JwtConfig> = LazyLock::new(|| JwtConfig {
    exp_seconds: std::env::var("JWT_EXP").expect("JWT_EXP must be set").parse().expect("JWT_EXP must be a number"),
    secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
});

pub struct DefaultJwtService;

impl super::IJwtService for DefaultJwtService {
    fn generate_jwt_token(user_id: i32, role: UserRoleType) -> String {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        let claims = Claims { sub: user_id.to_string(), exp: now + CONFIG.exp_seconds, role };
        encode(&Header::default(), &claims, &EncodingKey::from_secret(CONFIG.secret.as_bytes()))
            .expect("token generation should not fail with a valid secret")
    }

    fn generate_refresh_token() -> String { random_hex(32) }
    fn generate_api_token() -> String { random_hex(64) }
    
    fn get_secret<'a>() -> &'a str {
        &CONFIG.secret
    }
}

fn random_hex(n: usize) -> String {
    let mut bytes = vec![0u8; n];
    rand::rngs::OsRng.try_fill_bytes(&mut bytes).expect("OS RNG should not fail");
    hex::encode(bytes)
}
