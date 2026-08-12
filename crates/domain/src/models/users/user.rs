use chrono::{DateTime, Utc};
use super::enums::user_role_type::UserRoleType;
use crate::{aggregate_root::IAggregateRoot, models::base_entity::{BaseEntity, IBaseEntity}};

pub struct User {
    base: BaseEntity<i32>,
    first_name: String,
    last_name: String,
    email: String,
    password_hash: String,
    role: UserRoleType,
    is_active: bool,
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
            base: BaseEntity::new(),
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
    pub fn role(&self) -> UserRoleType { self.role }
    pub fn is_active(&self) -> bool { self.is_active }

    pub fn set_password_hash(&mut self, value: String) { self.password_hash = value }
    pub fn set_first_name(&mut self, value: String) { self.first_name = value }
    pub fn set_last_name(&mut self, value: String) { self.last_name = value }
    pub fn set_email(&mut self, value: String) { self.email = value }
    pub fn new_model(
        id: i32,
        first_name: String,
        last_name: String,
        email: String,
        password_hash: String,
        role: UserRoleType,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self { base: BaseEntity::new_model(id, created_at, updated_at), first_name, last_name, email, password_hash, role, is_active }
    }
}

impl IBaseEntity<i32> for User {
    fn id(&self) -> Option<i32> { self.base.id }
    fn base(&self) -> &BaseEntity<i32> { &self.base }
    fn created_at(&self) -> &DateTime<Utc> { &self.base.created_at }
    fn updated_at(&self) -> &DateTime<Utc> { &self.base.updated_at }
    
    fn touch(&mut self) {
        self.base.touch();
    }
}

impl IAggregateRoot<i32> for User {}