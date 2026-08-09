use chrono::{DateTime, Utc};

use crate::base_query_entity::{BaseQueryEntity, IBaseQueryEntity};

pub struct Product {
    base: BaseQueryEntity<i32>,
    name: String,
    price: i64,
    is_active: bool,
    user_id: i32,
}

impl Product {
    pub fn new(name: &str, price: i64, is_active: bool, user_id: i32) -> Self {
        Self {
            base: BaseQueryEntity::new(),
            name: name.to_string(),
            price: price,
            is_active: is_active,
            user_id: user_id
        }
    }
    pub fn name(&self) -> &str { &self.name }
    pub fn price(&self) -> i64 { self.price }
    pub fn is_active(&self) -> bool { self.is_active }
    pub fn user_id(&self) -> i32 { self.user_id }
}

impl IBaseQueryEntity<i32> for Product {
    fn id(&self) -> i32 { self.base.id.unwrap_or_default() }
    fn created_at(&self) -> &DateTime<Utc> { &self.base.created_at }
    fn updated_at(&self) -> &DateTime<Utc> { &self.base.updated_at }
    fn base(&self) -> &BaseQueryEntity<i32> { &self.base }
}