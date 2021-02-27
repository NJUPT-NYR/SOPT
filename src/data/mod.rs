pub mod user;
pub mod invitation;
pub mod torrent_info;
pub mod user_info;
// pub mod torrent;

use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct GeneralResponse {
    data: serde_json::Value,
    success: bool,
    #[serde(rename="errMsg")]
    err_msg: String,
}

impl GeneralResponse {
    pub fn from_err(err_msg: &str) -> Self {
        GeneralResponse {
            data: serde_json::from_str("{}").unwrap(),
            success: false,
            err_msg: String::from(err_msg),
        }
    }
}

pub trait ToResponse: Serialize {
    fn to_json(&self) -> GeneralResponse {
        let json_val = serde_json::to_value(self).expect("unable to parse to json");
        GeneralResponse {
            data: json_val,
            success: true,
            err_msg: "".to_string(),
        }
    }
}
