use super::*;

pub async fn update_or_add_activation(client: &sqlx::PgPool, id: i64, code: &str) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO activation(id, code) \
        VALUES($1, $2) ON CONFLICT (id) DO \
        UPDATE SET code = $2;",
        id,
        code
        )
        .execute(client)
        .await?;

    Ok(())
}

pub async fn find_activation_by_id(client: &sqlx::PgPool, id: i64) -> ActivationRet {
    sqlx::query_as!(
        Activation,
        "SELECT * FROM activation \
        WHERE id = $1;",
        id
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

pub async fn update_activated_by_id(client: &sqlx::PgPool, id: i64) -> Result<(), Error> {
    sqlx::query!(
        "WITH ret AS (UPDATE activation SET used = TRUE \
        WHERE id = $1) \
        UPDATE users SET activated = TRUE \
        WHERE id = $1;",
        id
        )
        .execute(client)
        .await?;

    Ok(())
}