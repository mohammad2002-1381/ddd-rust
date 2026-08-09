use axum::Router;

use crate::{controllers::admin_controllers::users_controller::IUsersController, state::SharedState};

pub mod users_controller;

pub trait IAdminController {
    fn admin_router(self) -> Router<SharedState>;
}

impl IAdminController for Router<SharedState> {
    fn admin_router(self) -> Router<SharedState> {
        self
            .nest("/users", Router::new().users_router())
    }
}