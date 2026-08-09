use serde::{Deserialize, Serialize};

use crate::mapper::IMapper;

#[derive(Serialize, Deserialize)]
pub struct PaginationParams {
    pub page_number: u64,
    pub page_size: u64,
}

impl PaginationParams {
    pub fn new(
        page_number: u64,
        page_size: u64,
    ) -> Self {
        Self { page_number, page_size }
    }
}

#[derive(Serialize)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total_count: u64,
    pub page_number: u64,
    pub page_size: u64
}

impl<E> PaginatedResult<E> {
    pub fn project_to<T>(self) -> PaginatedResult<T>
    where E: IMapper<T> {
        PaginatedResult {
            items: self.items
                    .into_iter()
                    .map(|item| item.map()) // item is &Model, so IMapper<T> must be implemented for &Model
                    .collect(),
            page_number: self.page_number,
            page_size: self.page_size,
            total_count: self.total_count
        }
    }
}