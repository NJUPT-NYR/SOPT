mod user;
mod invitation;
mod torrent;
mod admin;
pub(crate) mod config;
mod tracker;

use actix_web::{HttpResponse, *};
use serde::Deserialize;
use crate::error::{Error, error_string};
use crate::config::CONFIG;
use crate::util::*;
use crate::data::{Claim, ToResponse, GeneralResponse, DataWithCount};
use crate::controller::config::*;

/// A wrapper of Error so to reduce panic
/// and make HttpError more smooth
type HttpResult = Result<HttpResponse, Error>;

/// get username in jwt token
fn get_info_in_token(req: HttpRequest) -> Result<Claim, Error> {
    let auth = req.headers().get("Authorization");
    if auth.is_none() {
        return Err(Error::AuthError)
    }
    let data: Vec<&str> = auth.unwrap().to_str().map_err(error_string)?.split("Bearer").collect();
    let token = data[1].trim();

    let secret = CONFIG.secret_key.as_bytes();
    Ok(decode_and_verify_jwt(token, secret)?)
}

/// since most of cases are the need of username
fn get_name_in_token(req: HttpRequest) -> Result<String, Error> {
    Ok(get_info_in_token(req)?.sub)
}

pub(crate) fn api_service() -> Scope {
    web::scope("/api")
        .service(user::user_service())
        .service(invitation::invitation_service())
        .service(torrent::torrent_service())
        .service(admin::admin_service())
        .service(tracker::tracker_service())
}