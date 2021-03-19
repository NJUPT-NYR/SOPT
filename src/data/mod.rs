pub mod invitation;
pub mod torrent_info;
pub mod user;
pub mod user_info;
pub mod tag;
pub mod torrent;
pub mod rank;

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::error::Error;
use sopt_derive::ToResponse;
use std::collections::HashSet;

/// General Response structure used to
/// communicate with frontends.
///
/// it contains:
/// 1. data, returned data or null
/// 2. success, the status of request
/// 3. errMsg, not so severe errors prompt
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralResponse {
    pub data: serde_json::Value,
    pub success: bool,
    #[serde(rename = "errMsg")]
    pub err_msg: String,
}

impl GeneralResponse {
    /// takes an error &str and return a `GeneralResponse` struct
    pub fn from_err(err_msg: &str) -> Self {
        GeneralResponse {
            data: serde_json::from_str("null").unwrap(),
            success: false,
            err_msg: String::from(err_msg),
        }
    }
}

impl Default for GeneralResponse {
    /// default success value with data is `null`
    fn default() -> Self {
        GeneralResponse {
            data: serde_json::from_str("null").unwrap(),
            success: true,
            err_msg: String::from(""),
        }
    }
}

/// A trait used to automated Json Response constructions.
/// It demands the type implemented `Serialize` trait.
pub trait ToResponse: Serialize {
    /// common wrapper for data.
    /// use serde_json to serialize into a `GeneralResponse` struct
    fn to_json(&self) -> GeneralResponse {
        let json_val = serde_json::to_value(self)
            // never happens
            .expect("unable to parse to json");
        GeneralResponse {
            data: json_val,
            success: true,
            err_msg: "".to_string(),
        }
    }
}

impl ToResponse for String {}
impl ToResponse for HashSet<String> {}

/// A common wrapper used to return page count with list
#[derive(Serialize, Debug, ToResponse)]
pub struct DataWithCount {
    pub count: i64,
    pub ret: serde_json::Value,
}

impl DataWithCount {
    pub fn new(ret: serde_json::Value, count: i64) -> Self {
        DataWithCount {
            count,
            ret,
        }
    }
}

/// custom jwt struct
/// for now we need its username and role
#[derive(Serialize, Deserialize, Debug)]
pub struct Claim {
    pub sub: String,
    pub role: i64,
    pub exp: u64,
}

type CountRet = Result<i64, Error>;