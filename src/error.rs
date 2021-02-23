use actix_web::{HttpResponse, ResponseError};
use tokio_postgres::error::Error as PGError;
use std::fmt::{Display, Formatter};
use derive_more::From;

#[derive(From, Debug)]
pub enum Error {
    CookieError,
    OtherError(String),
    PGError(PGError),
    PoolError(deadpool_postgres::PoolError),
    RedisPoolError(deadpool_redis::PoolError),
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
            Error::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            },
            Error::RedisPoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            },
            Error::PGError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            },
            Error::OtherError(ref err) => {
                HttpResponse::InternalServerError().body(err)
            },
            Error::CookieError => HttpResponse::Ok().body("not login yet"),
        }
    }
}

pub fn error_string<T>(err: T) -> Error where
    T: std::error::Error,
{
    Error::OtherError(err.to_string())
}
