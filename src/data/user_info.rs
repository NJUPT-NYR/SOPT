use std::convert::TryFrom;
use super::*;

type UserInfoRet = Result<UserInfo, Error>;
type SlimUserInfoRet = Result<SlimUserInfo, Error>;

#[repr(C)]
#[derive(Deserialize, Debug)]
pub enum Level {
    Public = 0,
    OnlyFriend,
    Hidden,
}

impl TryFrom<i32> for Level {
    type Error = Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Level::Public as i32 => Ok(Level::Public),
            x if x == Level::OnlyFriend as i32 => Ok(Level::OnlyFriend),
            x if x == Level::Hidden as i32 => Ok(Level::Hidden),
            _ => Err(Error::OtherError("unknown privacy level".to_string())),
        }
    }
}

/// A full user info struct contains
/// 1. username: for performant issue
/// 2. register_time
/// 3. last_activity: updated when login
/// 4. invitor
/// 5. upload count, in bytes
/// 6. download count, in bytes
/// 7. money
/// 8. rank: an integer from 0
/// 9. avatar: b64encoded picture
/// 10. json values to store user defined columns
/// 11. privacy: whether show info in public
#[derive(Serialize, Debug, ToResponse)]
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
    pub privacy: i32,
}

/// Slim version user, mainly for privacy and performance.
#[derive(Serialize, Debug, ToResponse)]
pub struct SlimUserInfo {
    pub id: i64,
    pub username: String,
    pub rank: i32,
    pub avatar: Option<String>,
}

#[derive(Serialize, Debug, ToResponse)]
pub struct JoinedUser {
    pub info: UserInfo,
    pub account: Option<super::user::SlimUser>,
}

/// Add a full user info when sign up
pub async fn add_user_info(client: &sqlx::PgPool, id: i64, username: &str) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO user_info(id, username, register_time, last_activity) \
        VALUES($1, $2, NOW(), NOW());",
        id,
        username
        )
        .execute(client)
        .await?;

    Ok(())
}

/// Update last activity when login
pub async fn update_activity_by_name(client: &sqlx::PgPool, username: &str) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE user_info SET last_activity = now() \
        WHERE username = $1;",
        username
        )
        .execute(client)
        .await?;

    Ok(())
}

/// add invitor when sign up
pub async fn add_invitor_by_name(client: &sqlx::PgPool, username: &str, invitor: Option<String>) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE user_info SET invitor = $1 \
        WHERE username = $2;",
        invitor,
        username
        )
        .execute(client)
        .await?;

    Ok(())
}

/// transfer money with one sql
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

/// award money to some users
pub async fn award_money_by_id(client: &sqlx::PgPool, ids: &Vec<i64>, amount: f64) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE user_info SET money = money + $1 \
        WHERE id = ANY($2);",
        amount,
        ids
        )
        .execute(client)
        .await?;

    Ok(())
}

/// update money of a user
pub async fn update_money_by_name(client: &sqlx::PgPool, username: &str, amount: f64) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE user_info SET money = money + $1 \
        WHERE username = $2;",
        amount,
        username
        )
        .execute(client)
        .await?;

    Ok(())
}

/// Update user define columns, replace all without any check
pub async fn update_other_by_name(client: &sqlx::PgPool, username: &str, info: serde_json::Value) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE user_info SET other = $1 \
        WHERE username = $2;",
        info,
        username
        )
        .execute(client)
        .await?;

    Ok(())
}

/// Insert user uploaded avatar
pub async fn update_avatar_by_name(client: &sqlx::PgPool, username: &str, avatar: &str) -> Result<(), Error> {
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

/// change privacy level
pub async fn update_privacy_by_name(client: &sqlx::PgPool, username: &str, privacy: Level) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE user_info SET privacy = $1 \
        WHERE username = $2;",
        privacy as i32,
        username
        )
        .execute(client)
        .await?;

    Ok(())
}

/// find user_info with username
pub async fn find_user_info_by_name(client: &sqlx::PgPool, username: &str) -> UserInfoRet {
    sqlx::query_as!(
        UserInfo,
        "SELECT * FROM user_info \
        WHERE username = $1",
        username
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

/// find user info with username, return the slim one
pub async fn find_user_info_by_name_slim(client: &sqlx::PgPool, username: &str) -> SlimUserInfoRet {
    sqlx::query_as!(
        SlimUserInfo,
        "SELECT id, username, rank, avatar FROM user_info \
        WHERE username = $1",
        username
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}
