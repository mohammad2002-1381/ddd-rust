pub mod models;
pub mod mapper;
pub mod queries;
pub mod dto;
pub mod base_query_entity;
pub mod base_query_service;
pub mod query_request;
pub mod queryable;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
