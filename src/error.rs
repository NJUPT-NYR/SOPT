use crate::data::GeneralResponse;
use actix_web::{HttpResponse, ResponseError};
use deadpool_redis::PoolError;
use derive_more::From;
use sqlx::Error as DBError;
use std::fmt::{Display, Formatter};

/// General error types. it contains:
/// 1. CookieError, triggered when cookie is not set
/// 2. RedisError, wrapper for deadpool_redis `PoolError`
/// 3. DBError, wrapper for sqlx `Error`
/// 4. OtherError, with a generic to rewrite other errors to string
///     like utilities and standard library error.
///
/// All errors will be transformed to Http Response so no panic will happen.
#[derive(From, Debug)]
pub enum Error {
    CookieError,
    OtherError(String),
    RedisError(PoolError),
    DBError(DBError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

impl ResponseError for Error {
    /// Transform error messages to Http Response.
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::RedisError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
            Error::OtherError(ref err) => HttpResponse::InternalServerError().body(err),
            Error::DBError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
            Error::CookieError => {
                HttpResponse::Unauthorized().json(GeneralResponse::from_err("not login yet"))
            }
        }
    }
}

/// A generic function to be applied to any
/// type which implements Error trait.
/// They will be turned into strings of OtherError.
///
/// Especially useful when comes to `map_err()` case.
pub fn error_string<T>(err: T) -> Error
where
    T: std::error::Error,
{
    Error::OtherError(err.to_string())
}
