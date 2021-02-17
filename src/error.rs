use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;
use std::fmt::{Display, Formatter};
use derive_more::From;

#[derive(From, Debug)]
pub enum Error {
    NotFound,
    PGMError(PGMError),
    PGError(PGError),
    PoolError(PoolError),
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
            Error::NotFound => HttpResponse::NotFound().finish(),
            Error::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            },
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}


