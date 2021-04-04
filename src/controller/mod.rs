mod admin;
mod config;
mod invitation;
#[cfg(feature = "message")]
mod message;
mod torrent;
mod tracker;
mod user;

use crate::config::CONFIG;
pub use crate::controller::config::ALLOWED_DOMAIN;
use crate::controller::config::*;
use crate::data::*;
use crate::error::{error_string, Error};
use crate::rocksdb::{put_cf, ROCKSDB};
use crate::search::TORRENT_SEARCH_ENGINE;
use crate::util::*;
use crate::{get_from_config_cf, get_from_config_cf_untyped};
use actix_web::{HttpResponse, *};
use serde::Deserialize;
use std::convert::TryInto;

/// A macro used to make load data
/// from `rocksdb` smoother.
#[macro_export]
macro_rules! get_from_config_cf {
    ($s:literal, $t:ty) => {
        <$t>::from_ne_bytes(
            ROCKSDB
                .get_cf(ROCKSDB.cf_handle("config").unwrap(), $s)?
                .unwrap()
                .as_slice()
                .split_at(std::mem::size_of::<$t>())
                .0
                .try_into()
                .unwrap(),
        )
    };
}

/// A macro used to load data from
/// `rocksdb` and returns a string
#[macro_export]
macro_rules! get_from_config_cf_untyped {
    ($s:literal) => {
        String::from_utf8(
            ROCKSDB
                .get_cf(ROCKSDB.cf_handle("config").unwrap(), $s)?
                .unwrap(),
        )
        .map_err(error_string)?
    };
}

/// A wrapper of Error so to reduce panic
/// and make HttpError more smooth
type HttpResult = Result<HttpResponse, Error>;

/// get username in jwt token
fn get_info_in_token(req: &HttpRequest) -> Result<Claim, Error> {
    let auth = req.headers().get("Authorization");
    if auth.is_none() {
        return Err(Error::AuthError);
    }
    let data: Vec<&str> = auth
        .unwrap()
        .to_str()
        .map_err(error_string)?
        .split("Bearer")
        .collect();
    let token = data[1].trim();

    let secret = CONFIG.secret_key.as_bytes();
    decode_and_verify_jwt(token, secret)
}

/// since most of cases are the need of username
fn get_name_in_token(req: &HttpRequest) -> Result<String, Error> {
    Ok(get_info_in_token(req)?.sub)
}

pub fn api_service() -> Scope {
    let mut scope = web::scope("/api")
        .service(user::user_service())
        .service(invitation::invitation_service())
        .service(torrent::torrent_service())
        .service(admin::admin_service())
        .service(tracker::tracker_service());

    #[cfg(feature = "message")]
    {
        scope = scope.service(message::message_service());
    }

    scope
}
