use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Copy, Debug)]
pub struct BaseEntity<Id> {
    pub id: Option<Id>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<Id> BaseEntity<Id>  {
    pub fn new() -> Self {
        let now = Utc::now();
        Self { id: None, created_at: now, updated_at: now }
    }
    pub fn new_model(id: Id, created_at: DateTime<Utc>, updated_at: DateTime<Utc>) -> Self {
        Self { id: Some(id), created_at, updated_at }
    }
    pub fn set_id(&mut self, id: Id) { self.id = Some(id); }
    pub fn touch(&mut self) { self.updated_at = Utc::now() }
}

pub trait IBaseEntity<Id> {
    fn id(&self) -> Option<Id>;
    fn base(&self) -> &BaseEntity<Id>;
    fn created_at(&self) -> &DateTime<Utc>;
    fn updated_at(&self) -> &DateTime<Utc>;

    fn touch(&mut self);
}