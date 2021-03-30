use super::*;

pub async fn add_message(client: &sqlx::PgPool, sender: &str, receiver: &str, title: &str, body: Option<&str>) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO message(sender, receiver, title, body, sendTime) \
        VALUES($1, $2, $3, $4, NOW());",
        sender,
        receiver,
        title,
        body
        )
        .execute(client)
        .await?;

    Ok(())
}

pub async fn read_message(client: &sqlx::PgPool, ids: &[i64], receiver: &str) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE message SET read = TRUE \
        WHERE id = ANY($1) AND receiver = $2;",
        ids,
        receiver
        )
        .execute(client)
        .await?;

    Ok(())
}

/// prevent the hacker so we add sender
pub async fn delete_message_by_sender(client: &sqlx::PgPool, ids: &[i64], sender: &str) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE message SET visibleSender = FALSE \
        WHERE id = ANY($1) AND sender = $2;",
        ids,
        sender
        )
        .execute(client)
        .await?;

    Ok(())
}

pub async fn delete_message_by_receiver(client: &sqlx::PgPool, ids: &[i64], receiver: &str) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE message SET visibleReceiver = FALSE \
        WHERE id = ANY($1) AND receiver = $2;",
        ids,
        receiver
        )
        .execute(client)
        .await?;

    Ok(())
}

pub async fn list_sent_message(client: &sqlx::PgPool, sender: &str) -> MessageVecRet {
    Ok(sqlx::query_as!(
        Message,
        "SELECT id, sender, receiver, title, body, read, sendTime \
        FROM message WHERE sender = $1 AND visibleSender = TRUE;",
        sender
        )
        .fetch_all(client)
        .await?)
}

pub async fn list_received_message(client: &sqlx::PgPool, receiver: &str) -> MessageVecRet {
    Ok(sqlx::query_as!(
        Message,
        "SELECT id, sender, receiver, title, body, read, sendTime \
        FROM message WHERE receiver = $1 AND visibleReceiver = TRUE;",
        receiver
        )
        .fetch_all(client)
        .await?)
}
