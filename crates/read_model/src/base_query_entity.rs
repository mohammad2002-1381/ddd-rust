use chrono::{DateTime, Utc};

#[derive(Clone, Copy, Debug)]
pub struct BaseQueryEntity<Id> {
    pub id: Option<Id>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<Id> BaseQueryEntity<Id>  {
    pub fn new() -> Self {
        let now = Utc::now();
        Self { id: None, created_at: now, updated_at: now }
    }
    pub fn set_id(&mut self, id: Id) { self.id = Some(id); }
    pub fn touch(&mut self) { self.updated_at = Utc::now() }
}

pub trait IBaseQueryEntity<Id> {
    fn id(&self) -> Id;
    fn base(&self) -> &BaseQueryEntity<Id>;
    fn created_at(&self) -> &DateTime<Utc>;
    fn updated_at(&self) -> &DateTime<Utc>;
}