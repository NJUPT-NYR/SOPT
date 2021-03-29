use super::*;

pub async fn add_invitation_code(client: &sqlx::PgPool, sender: &str, code: &str, send_to: &str) -> InvitationRet {
    Ok(sqlx::query_as!(
        Invitation,
        "INSERT INTO invitations(sender, code, address) \
        VALUES ($1, $2, $3) RETURNING *;",
        sender,
        code,
        send_to,
        )
        .fetch_one(client)
        .await?)
}

pub async fn find_invitation_by_user(client: &sqlx::PgPool, username: &str) -> InvitationVecRet {
    Ok(sqlx::query_as!(
        Invitation,
        "SELECT * FROM invitations \
        WHERE sender = $1;",
        username,
        )
        .fetch_all(client)
        .await?)
}

pub async fn find_invitation_by_code(client: &sqlx::PgPool, code: &str) -> InvitationVecRet {
    Ok(sqlx::query_as!(
        Invitation,
        "SELECT * FROM invitations \
        WHERE code = $1;",
        code,
        )
        .fetch_all(client)
        .await?)
}
