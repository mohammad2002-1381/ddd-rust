use read_model::base_query_service::errors::QueryError;

use crate::common::error::CommandError;

pub mod current_user_service;
pub mod current_user_request;

pub type Error = CommandError;

pub trait ICurrentUserService: Send + Sync {
    fn user_id(&self) -> Result<i32, Error>;
}

impl From<Error> for QueryError {
    fn from(value: Error) -> Self {
        match value {
            Error::NotFound(_) => QueryError::NotFound,
            Error::Internal(msg) => QueryError::Database(msg),
            Error::Validation(msg) => QueryError::Mapping(msg),
            Error::Unauthorized(msg) => QueryError::Database(msg),
            Error::ExternalService(msg) => QueryError::Database(msg),
            CommandError::Forbidden(msg) => QueryError::Forbidden(msg),
            CommandError::Conflict(msg) => QueryError::Mapping(msg),
        }
    }
}