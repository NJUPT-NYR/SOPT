use serde::{Deserialize, Serialize};
use deadpool_postgres::Client;
use tokio_postgres::Row;
use crate::error::Error;
use super::exec_cmd_and_map;

type UserRet = Result<User, Error>;
type UserVecRet = Result<Vec<User>, Error>;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub password: Option<String>,
    pub passkey: Option<String>,
}

impl User {
    pub fn new(email: String, username: String, password: String, passkey: String) -> Self {
        User {
            id: 114514,
            email,
            username,
            password: Some(password),
            passkey: Some(passkey),
        }
    }
}

fn get_general_ret_user(row: &Row) -> User {
    User {
        id: row.get(0),
        email: row.get(1),
        username: row.get(2),
        password: None,
        passkey: None,
    }
}

fn get_full_ret_user(row: &Row) -> User {
    User {
        id: row.get(0),
        email: row.get(1),
        username: row.get(2),
        password: Some(row.get(3)),
        passkey: Some(row.get(4)),
    }
}

pub async fn add_user(client: &Client, user: User) -> UserRet {
    exec_cmd_and_map(
        &client,
        &"INSERT INTO users(email, username, password, passkey) \
        VALUES ($1, $2, $3, $4) RETURNING id, email, username;",
        &[
            &user.email,
            &user.username,
            &user.password.unwrap(),
            &user.passkey.unwrap(),
        ],
        get_general_ret_user)
        .await?
        .pop()
        .ok_or(Error::OtherError("Database inconsistent".to_string()))
}

pub async fn find_user_by_username(client: &Client, username: &str) -> UserVecRet {
    Ok(exec_cmd_and_map(
        &client,
        &"SELECT id, email, username FROM users \
        WHERE username = $1;",
        &[
            &username,
        ],
        get_general_ret_user)
        .await?)
}

pub async fn find_user_by_username_full(client: &Client, username: &str) -> UserVecRet {
    Ok(exec_cmd_and_map(
        &client,
        &"SELECT * FROM users \
        WHERE username = $1;",
        &[
            &username,
        ],
        get_full_ret_user)
        .await?)
}

pub async fn find_user_by_email(client: &Client, email: &str) -> UserVecRet {
    Ok(exec_cmd_and_map(
        &client,
        &"SELECT id, email, username FROM users \
        WHERE email = $1;",
        &[
            &email,
        ],
        get_general_ret_user)
        .await?)
}

pub async fn update_password_by_username(client: &Client, username: &str, new_pass: &str) -> UserRet {
    exec_cmd_and_map(
        &client,
        &"UPDATE users SET password = $1 \
         WHERE username = $2 RETURNING *;",
        &[
            &new_pass,
            &username,
        ],
        get_full_ret_user)
        .await?
        .pop()
        .ok_or(Error::OtherError("Database inconsistent".to_string()))
}

pub async fn update_passkey_by_username(client: &Client, username: &str, new_key: &str) -> UserRet {
    exec_cmd_and_map(
        &client,
        &"UPDATE users SET passkey = $1 \
         WHERE username = $2 RETURNING *;",
        &[
            &new_key,
            &username,
        ],
        get_full_ret_user)
        .await?
        .pop()
        .ok_or(Error::OtherError("Database inconsistent".to_string()))
}