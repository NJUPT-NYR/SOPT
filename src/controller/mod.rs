mod user;
mod invitation;

use actix_web::{HttpResponse, Error, Scope, web};

pub type HttpResult = Result<HttpResponse, Error>;

pub fn api_service() -> Scope {
    web::scope("/api")
        .service(user::user_service())
        .service(invitation::invitation_service())
}