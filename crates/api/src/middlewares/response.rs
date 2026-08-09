use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ApiResponse<T> {
    pub status: StatusCode,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            status: StatusCode::OK,
            data,
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            status: StatusCode::CREATED,
            data,
        }
    }
}

impl ApiResponse<()> {
    pub fn no_content() -> Self {
        Self {
            status: StatusCode::NO_CONTENT,
            data: (),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        if self.status == StatusCode::NO_CONTENT {
            return self.status.into_response();
        }

        (self.status, Json(self.data)).into_response()
    }
}