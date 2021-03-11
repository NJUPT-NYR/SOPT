use super::*;

type UserInfoRet = Result<UserInfo, Error>;
type SlimUserInfoRet = Result<SlimUserInfo, Error>;

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
}

/// Slim version user, mainly for list and performance.
#[derive(Serialize, Debug, ToResponse)]
pub struct SlimUserInfo {
    pub id: i64,
    pub username: String,
    pub last_activity: DateTime<Utc>,
    pub upload: i64,
    pub download: i64,
    pub money: f64
}

/// Add a full user info when sign up
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

/// Update last activity when login
pub async fn update_activity_by_name(client: &sqlx::PgPool, username: &str) -> SlimUserInfoRet {
    sqlx::query_as!(
        SlimUserInfo,
        "UPDATE user_info SET last_activity = now() \
        WHERE username = $1 RETURNING id, username, last_activity, upload, download, money;",
        username
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

/// add invitor when sign up
/// since additional checked is needed
pub async fn add_invitor_by_name(client: &sqlx::PgPool, username: &str, invitor: Option<String>) -> SlimUserInfoRet {
    sqlx::query_as!(
        SlimUserInfo,
        "UPDATE user_info SET invitor = $1 \
        WHERE username = $2 RETURNING id, username, last_activity, upload, download, money;",
        invitor,
        username
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
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

/// Update user define columns, replace all without any check
/// Returns the full user(for Debug use)
pub async fn update_other_by_name(client: &sqlx::PgPool, username: &str, info: serde_json::Value) -> UserInfoRet {
    sqlx::query_as!(
        UserInfo,
        "UPDATE user_info SET other = $1 \
        WHERE username = $2 RETURNING *;",
        info,
        username
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

/// Insert user uploaded avatar
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
