use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetUserPagedListQuery {
    pub page_number: u64,
    pub page_size: u64,
    pub disable_paging: Option<bool>
}

impl GetUserPagedListQuery {
    pub fn new(
        page_number: u64,
        page_size: u64,
        disable_paging: Option<bool>
    ) -> Self {
        Self { page_number, page_size, disable_paging }
    }
}
