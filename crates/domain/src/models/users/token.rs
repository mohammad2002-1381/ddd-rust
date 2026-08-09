use chrono::{DateTime, Months, Utc};

use crate::models::base_entity::{BaseEntity, IBaseEntity};

pub struct Token {
    base: BaseEntity<i32>,
    token: String,
    refresh_token: String,
    user_id: i32,
    expires: DateTime<Utc>,
}

impl Token {
    pub fn new(
        token: String,
        refresh_token: String,
        user_id: i32,
    ) -> Self {
        Self {
            base: BaseEntity::new(),
            token,
            refresh_token,
            expires: Utc::now()
                    .checked_add_months(Months::new(1))
                    .expect("valid date"),
            user_id
        }
    }

    pub fn token(&self) -> &str { &self.token }
    pub fn refresh_token(&self) -> &str { &self.refresh_token }
    pub fn user_id(&self) -> i32 { self.user_id }
    pub fn expires(&self) -> DateTime<Utc> { self.expires }
    pub fn new_model(
        id: i32,
        token: String,
        refresh_token: String,
        user_id: i32,
        expires: DateTime<Utc>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            base: BaseEntity::new_model(id, created_at, updated_at),
            token,
            refresh_token,
            user_id,
            expires,
        }   
    }
}

impl IBaseEntity<i32> for Token {
    fn id(&self) -> Option<i32> { self.base.id }
    fn base(&self) -> &BaseEntity<i32> { &self.base }
    fn created_at(&self) -> &DateTime<Utc> { &self.base.created_at }
    fn updated_at(&self) -> &DateTime<Utc> { &self.base.updated_at }
    
    fn touch(&mut self) {
        self.base.touch();
    }
}