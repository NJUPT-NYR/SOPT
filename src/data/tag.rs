use serde::Serialize;
use crate::error::Error;
use super::*;
use sopt::*;

type TagRet = Result<Tag, Error>;
type TagVecRet = Result<Vec<Tag>, Error>;

#[derive(Serialize, Debug, ToResponse)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub amount: i32,
}

pub async fn update_or_add_tag(client: &sqlx::PgPool, name: &str) -> TagRet {
    Ok(sqlx::query_as!(
        Tag,
        "INSERT INTO tag(name) VALUES($1) \
        ON CONFLICT (name) DO \
        UPDATE SET amount = tag.amount + 1 RETURNING *;",
        name
        )
        .fetch_one(client)
        .await?)
}

pub async fn decrease_amount_by_name(client: &sqlx::PgPool, name: &str) -> TagRet {
    sqlx::query_as!(
        Tag,
        "UPDATE tag SET amount = amount - 1 \
        WHERE name = $1 RETURNING *;",
        name
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

pub async fn find_hot_tag_by_amount(client: &sqlx::PgPool, num_want: i64) -> TagVecRet {

    Ok(sqlx::query_as!(
        Tag,
        "SELECT * FROM tag \
        ORDER BY amount DESC LIMIT $1;",
        num_want
        )
        .fetch_all(client)
        .await?)
}