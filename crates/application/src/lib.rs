use crate::services::init_services;

pub mod features;
pub mod common;
pub mod services;

pub async fn add_application() {
    let _ = init_services().await;
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
