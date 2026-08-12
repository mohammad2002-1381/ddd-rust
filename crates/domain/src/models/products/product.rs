use chrono::{DateTime, Utc};

use crate::{aggregate_root::IAggregateRoot, models::base_entity::{BaseEntity, IBaseEntity}};

pub struct Product {
    base: BaseEntity<i32>,
    name: String,
    price: i64,
    is_active: bool,
    user_id: i32,
}

impl Product {
    pub fn new(name: &str, price: i64, is_active: bool, user_id: i32) -> Self {
        Self {
            base: BaseEntity::new(),
            name: name.to_string(),
            price: price,
            is_active: is_active,
            user_id: user_id
        }
    }
    pub fn new_model(
        id: i32,
        name: String,
        price: i64,
        is_active: bool,
        user_id: i32,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            base: BaseEntity::new_model(id, created_at, updated_at),
            name,
            price,
            is_active,
            user_id,
        }
    }
    pub fn name(&self) -> &str { &self.name }
    pub fn price(&self) -> i64 { self.price }
    pub fn is_active(&self) -> bool { self.is_active }
    pub fn user_id(&self) -> i32 { self.user_id }
}

impl IBaseEntity<i32> for Product {
    fn id(&self) -> Option<i32> { self.base.id }
    fn created_at(&self) -> &DateTime<Utc> { &self.base.created_at }
    fn updated_at(&self) -> &DateTime<Utc> { &self.base.updated_at }
    fn base(&self) -> &BaseEntity<i32> { &self.base }
    
    fn touch(&mut self) {
        self.base.touch();
    }
}

impl IAggregateRoot<i32> for Product {}