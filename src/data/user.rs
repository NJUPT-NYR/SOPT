use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_pg_mapper::FromTokioPostgresRow;
use deadpool_postgres::Client;
use crate::error::Error;

type UserRet = Result<User, Error>;

#[derive(Deserialize, Serialize, PostgresMapper, Debug)]
#[pg_mapper(table = "users")]
pub struct User {
    pub email: String,
    pub username: String,
    pub password: String,
}

pub async fn add_user(client: &Client, user: User) -> UserRet {
    let cmd = client.prepare(
        &format!("INSERT INTO users(email, username, password)\
                        VALUES ($1, $2, $3) RETURNING {};",
                        &User::sql_table_fields())
    ).await.unwrap();

    client.query(
        &cmd,
        &[
            &user.email,
            &user.username,
            &user.password,
        ]).await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(Error::NotFound)
}
