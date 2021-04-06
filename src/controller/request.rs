use super::*;
use std::collections::HashMap;

// single wrapper
#[derive(Deserialize, Debug)]
pub struct IdWrapper {
    pub id: i64,
}
#[derive(Deserialize, Debug)]
pub struct IdsWrapper {
    pub ids: Vec<i64>,
}
#[derive(Deserialize, Debug)]
pub struct UsernameWrapper {
    pub username: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct EmailWrapper {
    pub email: String,
}
#[derive(Deserialize, Debug)]
pub struct NumWrapper {
    pub num: Option<usize>,
}
#[derive(Deserialize, Debug)]
pub struct PasswordWrapper {
    pub id: Option<i64>,
    pub code: Option<String>,
    pub password: String,
}
#[derive(Deserialize, Debug)]
pub struct InfoWrapper {
    pub info: serde_json::Value,
    pub privacy: i32,
}

// user
#[derive(Deserialize, Debug)]
pub struct SignUpRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub invite_code: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Debug)]
pub struct TransferRequest {
    pub to: String,
    pub amount: f64,
}
#[derive(Deserialize, Debug)]
pub struct ActivateRequest {
    pub id: i64,
    pub code: String,
}

// invitation and message
#[derive(Deserialize, Debug)]
pub struct InvitationRequest {
    pub to: String,
    pub address: String,
    pub body: String,
}
#[cfg(feature = "message")]
#[derive(Deserialize, Debug)]
pub struct MessageRequest {
    pub receiver: String,
    pub title: String,
    pub body: Option<String>,
}
#[cfg(feature = "message")]
#[derive(Deserialize, Debug)]
pub struct MessageDeleteRequest {
    pub ids: Vec<i64>,
    pub sender: bool,
}

// torrent
#[derive(Deserialize, Debug)]
pub enum Sort {
    Title,
    Poster,
    LastEdit,
    Length,
    Downloading,
    Uploading,
    Finished,
}
impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Deserialize, Debug, PartialEq)]
pub enum SortType {
    Asc,
    Desc,
}
#[derive(Deserialize, Debug)]
pub struct TorrentRequest {
    pub tags: Option<Vec<String>>,
    pub keywords: Option<Vec<String>>,
    pub page: Option<usize>,
    pub freeonly: Option<bool>,
    pub sort: Option<Sort>,
    #[serde(rename = "type")]
    pub sort_type: Option<SortType>,
}
#[derive(Deserialize, Debug)]
pub struct TorrentPostRequest {
    pub id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

// admin
#[derive(Deserialize, Debug)]
pub struct GroupAwardRequest {
    pub ids: Vec<i64>,
    pub amount: f64,
}
#[derive(Deserialize, Debug)]
pub struct EmailListRequest {
    pub add: Vec<String>,
    pub delete: Vec<String>,
}
#[derive(Deserialize, Debug)]
pub struct PermissionRequest {
    pub give: Vec<i32>,
    pub take: Vec<i32>,
    pub id: i64,
}
#[derive(Deserialize, Debug)]
pub struct SiteSettingRequest {
    pub settings: HashMap<String, String>,
}
