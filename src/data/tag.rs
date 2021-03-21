use super::*;

pub async fn update_or_add_tag(client: &sqlx::PgPool, name: &str) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO tag(name) VALUES($1) \
        ON CONFLICT (name) DO \
        UPDATE SET amount = tag.amount + 1;",
        name
        )
        .execute(client)
        .await?;

    Ok(())
}

pub async fn decrease_amount_by_name(client: &sqlx::PgPool, name: &str) -> Result<(), Error> {
    sqlx::query_as!(
        Tag,
        "UPDATE tag SET amount = amount - 1 \
        WHERE name = $1;",
        name
        )
        .execute(client)
        .await?;

    Ok(())
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