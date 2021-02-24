use serde::{Deserialize, Serialize};
use crate::error::Error;

type UserRet = Result<User, Error>;
type UserVecRet = Result<Vec<User>, Error>;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub password: String,
    pub passkey: String,
}

impl User {
    pub fn new(email: String, username: String, password: String, passkey: String) -> Self {
        User {
            id: 114514,
            email,
            username,
            password,
            passkey,
        }
    }
}

pub async fn add_user(client: &sqlx::PgPool, user: User) -> UserRet {
    Ok(sqlx::query_as!(
        User,
        "INSERT INTO users(email, username, password, passkey) \
        VALUES ($1, $2, $3, $4) RETURNING *;",
        user.email,
        user.username,
        user.password,
        user.passkey
        )
        .fetch_one(client)
        .await?)
}

pub async fn find_user_by_username(client: &sqlx::PgPool, username: &str) -> UserVecRet {
    Ok(sqlx::query_as!(
        User,
        "SELECT * FROM users \
        WHERE username = $1;",
        username,
        )
        .fetch_all(client)
        .await?)
}

pub async fn find_user_by_username_full(client: &sqlx::PgPool, username: &str) -> UserVecRet {
    Ok(sqlx::query_as!(
        User,
        "SELECT * FROM users \
        WHERE username = $1;",
        username
        )
        .fetch_all(client)
        .await?)
}

pub async fn find_user_by_email(client: &sqlx::PgPool, email: &str) -> UserVecRet {
    Ok(sqlx::query_as!(
        User,
        "SELECT * FROM users \
        WHERE email = $1;",
        email
        )
        .fetch_all(client)
        .await?)
}

pub async fn update_password_by_username(client: &sqlx::PgPool, username: &str, new_pass: &str) -> UserRet {
    Ok(sqlx::query_as!(
        User,
        "UPDATE users SET password = $1 \
         WHERE username = $2 RETURNING *;",
        new_pass,
        username
        )
        .fetch_one(client)
        .await?)
}

pub async fn update_passkey_by_username(client: &sqlx::PgPool, username: &str, new_key: &str) -> UserRet {
    Ok(sqlx::query_as!(
        User,
        "UPDATE users SET passkey = $1 \
         WHERE username = $2 RETURNING *;",
        new_key,
        username
        )
        .fetch_one(client)
        .await?)
}