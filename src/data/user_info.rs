use serde::{Deserialize, Serialize};
use crate::error::Error;
use chrono::{DateTime, Utc};
use super::*;
use sopt::*;

type UserInfoRet = Result<UserInfo, Error>;
type SlimUserInfoRet = Result<SlimUserInfo, Error>;

#[derive(Serialize, Deserialize, Debug, ToResponse)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub register_time: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub invitor: Option<String>,
    pub upload: i64,
    pub download: i64,
    pub money: f64,
    pub rank: i32,
    // b64encode
    pub avatar: Option<String>,
    pub other: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Debug, ToResponse)]
pub struct SlimUserInfo {
    pub id: i64,
    pub username: String,
    pub last_activity: DateTime<Utc>,
    pub upload: i64,
    pub download: i64,
    pub money: f64
}

impl UserInfo {
    #[allow(dead_code)]
    pub fn encode_avatar(&mut self, buf: Vec<u8>) {
        self.avatar = Some(base64::encode(buf));
    }
}

pub async fn add_user_info(client: &sqlx::PgPool, id: i64, username: &str) -> UserInfoRet {
    // the design of sqlx macro limits type casting
    Ok(sqlx::query_as!(
        UserInfo,
        "INSERT INTO user_info(id, username, register_time, last_activity) \
        VALUES($1, $2, NOW(), NOW()) RETURNING *;",
        id,
        username
        )
        .fetch_one(client)
        .await?)
}

pub async fn update_activity_by_name(client: &sqlx::PgPool, username: &str) -> SlimUserInfoRet {
    Ok(sqlx::query_as!(
        SlimUserInfo,
        "UPDATE user_info SET last_activity = now() \
        WHERE username = $1 RETURNING id, username, last_activity, upload, download, money;",
        username
        )
        .fetch_one(client)
        .await?)
}

pub async fn add_invitor_by_name(client: &sqlx::PgPool, username: &str, invitor: Option<String>) -> SlimUserInfoRet {
    Ok(sqlx::query_as!(
        SlimUserInfo,
        "UPDATE user_info SET invitor = $1 \
        WHERE username = $2 RETURNING id, username, last_activity, upload, download, money;",
        invitor,
        username
        )
        .fetch_one(client)
        .await?)
}

