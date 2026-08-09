use sea_orm::DbErr;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("not found")]
    NotFound,
    #[error("conflict: {0}")]
    Conflict(String),     // unique constraint / version conflict -> 409
    #[error("data integrity: {0}")]
    Mapping(String),      // stored row violates a domain invariant -> internal (500)
    #[error("database error: {0}")]
    Database(String),     // any other persistence failure -> internal (500)
}

impl From<DbErr> for RepositoryError {
    fn from(value: DbErr) -> Self {
        match value {
            DbErr::RecordNotFound(_) => RepositoryError::NotFound,
            other => RepositoryError::Database(other.to_string())
        }
    }
}