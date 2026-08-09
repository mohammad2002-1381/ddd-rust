use std::{env, sync::OnceLock, time::Duration};
use redis::{AsyncCommands, aio::MultiplexedConnection};
use serde::{Serialize, de::DeserializeOwned};

use crate::services::ServiceRegister;

static REDIS_CONN: OnceLock<MultiplexedConnection> = OnceLock::new();

fn conn() -> MultiplexedConnection {
    REDIS_CONN.get().expect("cache not initialized — call infrastructure::init() first").clone()
}

pub struct RedisCacheService;

impl ServiceRegister for RedisCacheService {
    async fn register_service() -> Result<(), crate::services::Error> {
        let redis_url = env::var("REDIS_URL").map_err(|_| crate::services::Error::MissingEnvVar("REDIS_URL not found".to_string()))?;
        let client = redis::Client::open(redis_url)
        .map_err(|e| crate::services::Error::ConnectionFailed(e.to_string()))?;
        let conn = client.get_multiplexed_async_connection().await
            .map_err(|e| crate::services::Error::ConnectionFailed(e.to_string()))?;
        REDIS_CONN.set(conn).map_err(|_| crate::services::Error::AlreadyInitialized("already initialized".into()))?;
        Ok(())
    }
}

impl super::ICacheService for RedisCacheService {
    async fn get<T: DeserializeOwned>(key: &str) -> Result<Option<T>, super::CacheError> {
        let mut c = conn();
        let raw: Option<String> = c.get(key).await
            .map_err(|e| super::CacheError::Internal(e.to_string()))?;  // manual map_err, NOT a `From` impl — see note below
        raw.map(|s| serde_json::from_str(&s).map_err(|e| super::CacheError::Internal(e.to_string()))).transpose()
    }

    async fn set<T: Serialize + Send + Sync>(key: &str, value: &T, ttl: Option<Duration>) -> Result<(), super::CacheError> {
        let mut c = conn();
        let json = serde_json::to_string(value).map_err(|e| super::CacheError::Internal(e.to_string()))?;
        let result: Result<(), redis::RedisError> = match ttl {
            Some(d) => c.set_ex(key, json, d.as_secs()).await,
            None => c.set(key, json).await,
        };
        result.map_err(|e| super::CacheError::Internal(e.to_string()))
    }

    async fn remove(key: &str) -> Result<(), super::CacheError> {
        conn().del::<_, ()>(key).await.map_err(|e| super::CacheError::Internal(e.to_string()))
    }
}