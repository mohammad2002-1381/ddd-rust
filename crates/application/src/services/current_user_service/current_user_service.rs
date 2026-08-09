use crate::services::jwt_service::default_jwt_service::Claims;
use super::ICurrentUserService;

pub struct CurrentUserService {
    claims: Option<Claims>,
}

impl CurrentUserService {
    pub fn new(claims: Option<Claims>) -> Self {
        Self { claims }
    }
}

impl ICurrentUserService for CurrentUserService {
    fn user_id(&self) -> Result<i32, super::Error> {
        let claims = self
            .claims
            .as_ref()
            .ok_or(super::Error::Unauthorized(
                "missing claims".to_string(),
            ))?;

        let user_id = claims
            .sub
            .parse::<i32>()
            .map_err(|_| super::Error::Unauthorized(
                "invalid user id in token".to_string(),
            ))?;

        Ok(user_id)
    }
}