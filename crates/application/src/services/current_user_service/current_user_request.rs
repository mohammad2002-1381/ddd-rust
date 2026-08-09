use axum::{extract::FromRequestParts, http::request::Parts};

use crate::services::{current_user_service::current_user_service::CurrentUserService, jwt_service::default_jwt_service::Claims};

pub struct CurrentUserRequest(pub CurrentUserService);

impl<S> FromRequestParts<S> for CurrentUserRequest
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let claims = parts.extensions.get::<Claims>().cloned();
        Ok(CurrentUserRequest(CurrentUserService::new(claims)))
    }
}
