use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use tokio_postgres::error::Error as PGError;
use std::fmt::{Display, Formatter};
use derive_more::From;

#[derive(From, Debug)]
pub enum Error {
    CookieError,
    OtherError,
    PGError(PGError),
    PoolError(PoolError),
    RedisError(deadpool_redis::PoolError),
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
                // TODO: add check for time error
                HttpResponse::InternalServerError().body(err.to_string())
            },
            Error::RedisError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            },
            Error::CookieError => HttpResponse::Ok().body("not login yet"),
            _ => HttpResponse::InternalServerError().body("unexpected error"),
        }
    }
}
