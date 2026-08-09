use axum::{extract::{FromRequest, Request}};
use serde::de::DeserializeOwned;

use crate::middlewares::error_exception::ApiError;

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
#[must_use]
pub struct JsonParam<T>(pub T);

impl<S, T> FromRequest<S> for JsonParam<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                Err(ApiError::Validation(rejection.body_text()))
            }
        }
    }
}
