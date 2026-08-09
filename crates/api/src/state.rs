// use infrastructure::repositories::{ProductRepository, UserRepository};
// use query_handler::query_services::{ProductQueryService, UserQueryService};

#[derive(Clone)]
pub struct AppState {

}

pub type SharedState = AppState;

impl AppState {
    pub async fn new() -> SharedState {
        // let command_db_connection = infrastructure::db_context::get_db_connection_async().await.unwrap();
        // let query_db_connection = query_handler::db_context::get_db_connection_async().await.unwrap();
        Self {
            // product_repository: Arc::new(ProductRepository::new(command_db_connection.clone())),
            // product_query_service: Arc::new(ProductQueryService::new(query_db_connection.clone())),

            // user_repository: Arc::new(UserRepository::new(command_db_connection.clone())),
            // user_query_service: Arc::new(UserQueryService::new(query_db_connection.clone())),
        }
    }
}
