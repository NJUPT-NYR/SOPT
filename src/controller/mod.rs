mod user;
mod invitation;
mod torrent;

use actix_web::{HttpResponse, *};
use serde::{Deserialize, Serialize};
use crate::error::{Error, error_string};
use crate::config::CONFIG;
use crate::util::*;

/// A wrapper of Error so to reduce panic
/// and make HttpError more smooth
pub type HttpResult = Result<HttpResponse, Error>;

/// get username in jwt token
pub fn get_name_in_token(req: HttpRequest) -> Result<String, Error> {
    let auth = req.headers().get("Authorization");
    if auth.is_none() {
        return Err(Error::AuthError)
    }
    let data: Vec<&str> = auth.unwrap().to_str().map_err(error_string)?.split("Bearer").collect();
    let token = data[1].trim();

    let secret = CONFIG.secret_key.as_bytes();
    Ok(crate::util::decode_and_verify_jwt(token, secret)?)
}

pub fn api_service() -> Scope {
    web::scope("/api")
        .service(user::user_service())
        .service(invitation::invitation_service())
        .service(torrent::torrent_service())
}