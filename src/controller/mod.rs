pub mod user;

use actix_web::{HttpResponse, Error, Scope, web};

pub type HttpResult = Result<HttpResponse, Error>;

pub fn api_service() -> Scope {
    web::scope("/api")
        .service(user::user_service())
}