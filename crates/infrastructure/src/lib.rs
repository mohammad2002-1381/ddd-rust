mod repository;
mod entity_configurations;
pub mod db_context;
pub mod queryable;
pub mod executable;

pub use repository::pg as repositories;

pub async fn add_infrastructure() {
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
