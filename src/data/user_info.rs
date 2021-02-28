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
    // use rank is for configurable name
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

pub async fn find_slim_info_by_name(client: &sqlx::PgPool, username: &str) -> SlimUserInfoRet {
    Ok(sqlx::query_as!(
        SlimUserInfo,
        "SELECT id, username, last_activity, upload, download, money FROM user_info \
        WHERE username = $1;",
        username
        )
        .fetch_one(client)
        .await?)
}

pub async fn transfer_money_by_name(client: &sqlx::PgPool, from: &str, to: &str, amount: f64) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE user_info SET money = CASE \
            WHEN username = $1 THEN money - $3\
            WHEN username = $2 THEN money + $3\
        END \
        WHERE username in ($1, $2)",
        from,
        to,
        amount
        )
        .execute(client)
        .await?;

    Ok(())
}

pub async fn update_other_by_name(client: &sqlx::PgPool, username: &str, info: serde_json::Value) -> UserInfoRet {
    Ok(sqlx::query_as!(
        UserInfo,
        "UPDATE user_info SET other = $1 \
        WHERE username = $2 RETURNING *;",
        info,
        username
        )
        .fetch_one(client)
        .await?)
}

pub async fn update_avatar_by_name(client: &sqlx::PgPool, username: &str, avatar: String) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE user_info SET avatar = $1 \
        WHERE username = $2;",
        avatar,
        username
        )
        .execute(client)
        .await?;

    Ok(())
}
