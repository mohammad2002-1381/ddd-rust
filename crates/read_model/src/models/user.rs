use chrono::{DateTime, Utc};
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

use crate::base_query_entity::{BaseQueryEntity, IBaseQueryEntity};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Deserialize, Serialize
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Text"
)]
pub enum UserRoleType {
    #[sea_orm(string_value = "admin")]
    Admin,

    #[sea_orm(string_value = "user")]
    User
}

pub struct User {
    base: BaseQueryEntity<i32>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub role: UserRoleType,
    pub is_active: bool,
}

impl User {
    pub fn new(
        first_name: &str,
        last_name: &str,
        email: &str,
        password_hash: &str,
        role: UserRoleType,
        is_active: bool
    ) -> Self {
        Self {
            base: BaseQueryEntity::new(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            role: role,
            is_active: is_active
        }
    }

    pub fn first_name(&self) -> &str { &self.first_name }
    pub fn last_name(&self) -> &str { &self.last_name }
    pub fn email(&self) -> &str { &self.email }
    pub fn password_hash(&self) -> &str { &self.password_hash }
}

impl IBaseQueryEntity<i32> for User {
    fn id(&self) -> i32 { self.base.id.unwrap_or_default() }
    fn base(&self) -> &BaseQueryEntity<i32> { &self.base }
    fn created_at(&self) -> &DateTime<Utc> { &self.base.created_at }
    fn updated_at(&self) -> &DateTime<Utc> { &self.base.updated_at }
}