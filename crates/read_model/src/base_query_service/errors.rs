use sea_orm::DbErr;

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error("not found")]
    NotFound,
    #[error("forbidden: {0}")]
    Forbidden(String),    // authenticated but not allowed to see this -> 403  (was lost)
    #[error("data integrity: {0}")]
    Mapping(String),      // row -> DTO failed -> internal (500)
    #[error("database error: {0}")]
    Database(String),     // query failed -> internal (500)
}

impl From<DbErr> for QueryError {
    fn from(value: DbErr) -> Self {
        match value {
            DbErr::RecordNotFound(_) => QueryError::NotFound,
            other => QueryError::Database(other.to_string())
        }
    }
}