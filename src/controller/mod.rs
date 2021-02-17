pub mod user;

use actix_web::{HttpResponse, Error};

pub type HttpResult = Result<HttpResponse, Error>;