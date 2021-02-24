mod user;
mod invitation;

use actix_web::{HttpResponse, Scope, web};
use crate::error::Error;
use sqlx::{postgres::Postgres, Pool};

pub type HttpResult = Result<HttpResponse, Error>;
pub type PgPool = Pool<Postgres>;

pub fn api_service() -> Scope {
    web::scope("/api")
        .service(user::user_service())
        .service(invitation::invitation_service())
}