use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use deadpool_postgres::Client;
use tokio_postgres::Row;
use crate::error::Error;
use crate::data::exec_cmd_and_map;

type UserRet = Result<User, Error>;
type UserVecRet = Result<Vec<User>, Error>;

#[derive(Deserialize, Serialize, PostgresMapper, Debug)]
#[pg_mapper(table = "users")]
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
        .ok_or(Error::NotFound)
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
