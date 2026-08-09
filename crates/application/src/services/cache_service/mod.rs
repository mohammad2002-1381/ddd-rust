pub mod default_cache_service;
pub mod redis_cache_service;

use std::time::Duration;
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug)]
pub enum CacheError {
    Connection(String),
    Internal(String),
}

// no &self — adapter is a ZST reaching into its own static
pub trait ICacheService {
    fn get<T: DeserializeOwned>(key: &str) -> impl Future<Output = Result<Option<T>, CacheError>> + Send;
    fn set<T: Serialize + Send + Sync>(key: &str, value: &T, ttl: Option<Duration>) -> impl Future<Output = Result<(), CacheError>> + Send;
    fn remove(key: &str) -> impl Future<Output = Result<(), CacheError>> + Send;
}