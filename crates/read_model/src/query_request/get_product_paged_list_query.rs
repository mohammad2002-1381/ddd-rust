use serde::{self, Deserialize};

#[derive(Deserialize)]
pub struct GetProductPagedListQuery {
    pub page_number: u64,
    pub page_size: u64,
    pub disable_paging: Option<bool>
}

impl GetProductPagedListQuery {
    pub fn new(
        page_number: u64,
        page_size: u64,
        disable_paging: Option<bool>
    ) -> Self {
        Self { page_number, page_size, disable_paging }
    }
}