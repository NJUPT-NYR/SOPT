mod admin;
mod config;
mod invitation;
#[cfg(feature = "message")]
mod message;
mod request;
mod torrent;
mod tracker;
mod user;

use crate::config::CONFIG;
use crate::controller::config::*;
pub use crate::controller::config::{ALLOWED_DOMAIN, STRING_SITE_SETTING};
use crate::data::kv::KVDB;
use crate::data::*;
use crate::deserialize_from_req;
use crate::error::{error_string, Error};
use crate::search::TORRENT_SEARCH_ENGINE;
use crate::util::*;
use actix_web::{HttpResponse, *};
use request::*;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[macro_export]
macro_rules! deserialize_from_req {
    ($s:expr, $t:ty) => {
        serde_qs::from_str::<$t>($s.uri().query().unwrap_or_default())
            .map_err(|e| Error::RequestError(e.to_string()))?
    };
}

/// A wrapper of `Error` so to reduce panic
/// and make `HttpError` more smooth
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
    if data.len() < 2 {
        return Err(Error::AuthError);
    }
    let token = data[1].trim();

    let secret = CONFIG.secret_key.as_bytes();
    decode_and_verify_jwt(token, secret)
}

/// since most of cases are the need of username
fn get_name_in_token(req: &HttpRequest) -> Result<String, Error> {
    Ok(get_info_in_token(req)?.sub)
}

#[derive(Serialize, Debug)]
struct UpdateFilter {
    set: Option<String>,
    delete: Option<String>,
}

async fn update_passkey_filter(set: Option<String>, delete: Option<String>) -> Result<(), Error> {
    let addr = format!("http://{}/tracker/update_filter", CONFIG.tracker_addr);
    let client = reqwest::Client::new();
    let query = UpdateFilter { set, delete };

    let resp = client
        .post(&addr)
        .json(&query)
        .send()
        .await
        .map_err(|e| Error::OtherError(e.to_string()))?;
    if !resp.status().is_success() {
        return Err(Error::OtherError("unable to set filter".to_string()));
    }

    Ok(())
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
