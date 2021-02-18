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
    pub email: String,
    pub username: String,
    pub password: Option<String>,
}

fn get_general_ret_user(row: &Row) -> User {
    User {
        email: row.get(0),
        username: row.get(1),
        password: None,
    }
}

pub async fn add_user(client: &Client, user: User) -> UserRet {
    exec_cmd_and_map(
        &client,
        &"INSERT INTO users(email, username, password) \
        VALUES ($1, $2, $3) RETURNING email, username;",
        &[
            &user.email,
            &user.username,
            &user.password
        ],
        get_general_ret_user)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

pub async fn find_user_by_username(client: &Client, username: &str) -> UserVecRet {
    Ok(exec_cmd_and_map(
        &client,
        &"SELECT email, username FROM users \
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
        &"SELECT email, username FROM users \
        WHERE email = $1;",
        &[
            &email,
        ],
        get_general_ret_user)
        .await?)
}
