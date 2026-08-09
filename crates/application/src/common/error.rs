use domain::base_repository::errors::RepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("validation: {0}")]
    Validation(String),
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("external service: {0}")]
    ExternalService(String),
    #[error("internal: {0}")]
    Internal(String),
}

impl From<RepositoryError> for CommandError {
    fn from(e: RepositoryError) -> Self {
        match e {
            RepositoryError::NotFound     => Self::NotFound("resource not found".into()),
            RepositoryError::Conflict(m)  => Self::Conflict(m),
            RepositoryError::Mapping(m)   => Self::Internal(m),
            RepositoryError::Database(m)  => Self::Internal(m),
        }
    }
}
