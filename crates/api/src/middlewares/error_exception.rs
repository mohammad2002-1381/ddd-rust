use application::common::error::CommandError;
use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use read_model::base_query_service::errors::QueryError;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
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

impl From<CommandError> for ApiError {
    fn from(err: CommandError) -> Self {
        match err {
            CommandError::NotFound(e) => ApiError::NotFound(e),
            CommandError::Validation(e) => ApiError::Validation(e),
            CommandError::Unauthorized(e) => ApiError::Unauthorized(e),
            CommandError::Forbidden(e) => ApiError::Forbidden(e),
            CommandError::Conflict(e) => ApiError::Conflict(e),
            CommandError::ExternalService(e) => ApiError::ExternalService(e),
            CommandError::Internal(e) => ApiError::Internal(e),
        }
    }
}

impl From<QueryError> for ApiError {
    fn from(value: QueryError) -> Self {
        value.into()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            ApiError::ExternalService(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg.clone()),
            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
        };

        (status, Json(ErrorResponse { error: error_message })).into_response()
    }
}
