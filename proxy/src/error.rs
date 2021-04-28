use actix_web::{HttpResponse, ResponseError};
use deadpool::managed::PoolError;
use deadpool_redis::redis::RedisError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ProxyError {
    RequestError(&'static str),
    RedisError,
    PoolError,
    EncodeError,
}

impl Display for ProxyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T> From<PoolError<T>> for ProxyError {
    fn from(_: PoolError<T>) -> Self {
        ProxyError::PoolError
    }
}

impl From<RedisError> for ProxyError {
    fn from(_: RedisError) -> Self {
        Self::RedisError
    }
}

impl From<bendy::encoding::Error> for ProxyError {
    fn from(_: bendy::encoding::Error) -> Self {
        Self::EncodeError
    }
}

impl From<serde_qs::Error> for ProxyError {
    fn from(_: serde_qs::Error) -> Self {
        Self::EncodeError
    }
}

impl std::error::Error for ProxyError {}

impl ResponseError for ProxyError {
    /// Transform error messages to Http Response.
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().body(format!("Error: {:?}", self))
    }
}
