mod entity_configurations;
mod query_service;
pub mod queryable;
pub mod db_context;

pub use query_service::pg as query_services;

pub async fn add_query_handler() {
    let _ = db_context::init_db().await;
}

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
