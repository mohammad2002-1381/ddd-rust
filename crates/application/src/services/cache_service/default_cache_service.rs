use std::{collections::HashMap, sync::LazyLock, time::Duration};
use tokio::sync::RwLock;
use serde::{Serialize, de::DeserializeOwned};

use crate::services::ServiceRegister;

static STORE: LazyLock<RwLock<HashMap<String, String>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

pub struct InMemoryCacheService;

impl ServiceRegister for InMemoryCacheService {
    async fn register_service() -> Result<(), crate::services::Error> {
        Ok(())
    }
}

impl super::ICacheService for InMemoryCacheService {
    async fn get<T: DeserializeOwned>(key: &str) -> Result<Option<T>, super::CacheError> {
        let store = STORE.read().await;
        store.get(key)
            .map(|json| serde_json::from_str(json).map_err(|e| super::CacheError::Internal(e.to_string())))
            .transpose()
    }

    async fn set<T: Serialize + Send + Sync>(key: &str, value: &T, _ttl: Option<Duration>) -> Result<(), super::CacheError> {
        let json = serde_json::to_string(value).map_err(|e| super::CacheError::Internal(e.to_string()))?;
        STORE.write().await.insert(key.to_string(), json);
        Ok(())
    }

    async fn remove(key: &str) -> Result<(), super::CacheError> {
        STORE.write().await.remove(key);
        Ok(())
    }
}