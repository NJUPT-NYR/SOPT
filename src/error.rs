use actix_web::{HttpResponse, ResponseError};
use sqlx::Error as DBError;
use deadpool_redis::PoolError;
use std::fmt::{Display, Formatter};
use derive_more::From;
use crate::data::GeneralResponse;

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
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::RedisError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
            Error::OtherError(ref err) => HttpResponse::InternalServerError().body(err),
            Error::DBError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
            Error::CookieError => HttpResponse::Ok().json(GeneralResponse::from_err("not login yet")),
        }
    }
}

pub fn error_string<T>(err: T) -> Error where
    T: std::error::Error,
{
    Error::OtherError(err.to_string())
}
