use axum::Router;
use crate::{controllers::{admin_controllers::IAdminController, product_controller::IProductController, user_controller::IUserController}, state::SharedState};

pub mod user_controller;
pub mod product_controller;
pub mod admin_controllers;

pub fn inject_routers(state: SharedState) -> Router {
    let api_routes = Router::new()
        .nest("/admin", Router::new().admin_router())
        .nest("/user", Router::new().user_router())
        .nest("/product", Router::new().product_router());

    Router::new()
        .nest("/api/v1", api_routes)
        .with_state(state)
}