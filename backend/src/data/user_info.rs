use super::*;
use std::convert::TryFrom;

/// privacy level, can be set by yourself
/// but respectively, you must make it functional
/// via updating some controllers
#[repr(C)]
#[derive(Deserialize, Debug)]
pub enum Level {
    Public = 0,
    Hidden,
}

impl TryFrom<i32> for Level {
    type Error = Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Level::Public as i32 => Ok(Level::Public),
            x if x == Level::Hidden as i32 => Ok(Level::Hidden),
            _ => Err(Error::OtherError("unknown privacy level".to_string())),
        }
    }
}

pub async fn find_user_info_by_name_mini(client: &sqlx::PgPool, username: &str) -> MiniUserRet {
    sqlx::query_as!(
        MiniUser,
        "SELECT id, upload, download, registerTime FROM user_info \
        WHERE username = $1",
        username
    )
    .fetch_all(client)
    .await?
    .pop()
    .ok_or(Error::NotFound)
}

pub async fn update_io_by_id(
    client: &sqlx::PgPool,
    id: i64,
    upload: i64,
    download: i64,
) -> MiniUserRet {
    sqlx::query_as!(
        MiniUser,
        "UPDATE user_info SET upload = upload + $1, download = download + $2 \
        WHERE id = $3 RETURNING id, upload, download, registerTime;",
        upload,
        download,
        id
    )
    .fetch_all(client)
    .await?
    .pop()
    .ok_or(Error::NotFound)
}

pub async fn find_user_info_by_name(client: &sqlx::PgPool, username: &str) -> UserRet {
    sqlx::query_as!(
        User,
        "WITH ret AS (\
            SELECT user_info.id, rank.name FROM rank INNER JOIN user_info ON rank.id = user_info.rank
            WHERE user_info.username = $1
        ) SELECT users.id, users.username, registerTime, lastActivity, invitor, upload, download, user_info.money, \
        ret.name as rank, avatar, other, privacy, email, passkey FROM user_info INNER JOIN users ON user_info.id = users.id \
        INNER JOIN ret ON user_info.id = ret.id WHERE user_info.username = $1",
        username
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

pub async fn update_activity_by_name(client: &sqlx::PgPool, username: &str) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE user_info SET lastActivity = now() \
        WHERE username = $1;",
        username
    )
    .execute(client)
    .await?;

    Ok(())
}

pub async fn add_invitor_by_name(
    client: &sqlx::PgPool,
    id: i64,
    invitor: Option<String>,
    code: &str,
) -> Result<(), Error> {
    sqlx::query!(
        "WITH rt AS (UPDATE user_info SET invitor = $1 WHERE id = $2) \
        UPDATE invitations SET usage = TRUE WHERE code = $3;",
        invitor,
        id,
        code
    )
    .execute(client)
    .await?;

    Ok(())
}

pub async fn transfer_money_by_name(
    client: &sqlx::PgPool,
    from: &str,
    to: &str,
    amount: f64,
) -> Result<(), Error> {
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

pub async fn award_money_by_id(
    client: &sqlx::PgPool,
    ids: &[i64],
    amount: f64,
) -> Result<(), Error> {
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

pub async fn update_money_by_name(
    client: &sqlx::PgPool,
    username: &str,
    amount: f64,
) -> Result<(), Error> {
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

pub async fn update_money_by_id(client: &sqlx::PgPool, id: i64, amount: f64) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE user_info SET money = money + $1 \
        WHERE id = $2;",
        amount,
        id
    )
    .execute(client)
    .await?;

    Ok(())
}

/// update user define columns, replace all without any check
pub async fn update_other_by_name(
    client: &sqlx::PgPool,
    username: &str,
    info: &serde_json::Value,
) -> Result<(), Error> {
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

pub async fn update_avatar_by_name(
    client: &sqlx::PgPool,
    username: &str,
    avatar: &str,
) -> Result<(), Error> {
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

pub async fn update_privacy_by_name(
    client: &sqlx::PgPool,
    username: &str,
    level: Level,
) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE user_info SET privacy = $1 \
        WHERE username = $2;",
        level as i32,
        username
    )
    .execute(client)
    .await?;

    Ok(())
}

pub async fn update_rank_by_name(
    client: &sqlx::PgPool,
    username: &str,
    rank: i32,
) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE user_info SET rank = $1 \
        WHERE username = $2;",
        rank,
        username
    )
    .execute(client)
    .await?;

    Ok(())
}
