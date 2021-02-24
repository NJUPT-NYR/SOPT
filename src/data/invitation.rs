use serde::{Deserialize, Serialize};
use super::*;
use crate::error::Error;

type InviteRet = Result<InvitationCode, Error>;
type InviteVecRet = Result<Vec<InvitationCode>, Error>;

#[derive(Deserialize, Serialize, Debug)]
pub struct InvitationCode {
    pub id: i64,
    pub sender: Option<String>,
    pub code: String,
    pub send_to: String,
    pub is_used: bool,
}

impl InvitationCode {
    pub fn new(sender: String, code: String, send_to: String) -> Self {
        InvitationCode {
            id: 1919810,
            sender: Some(sender),
            code,
            send_to,
            is_used: false,
        }
    }
}

pub async fn add_invitation_code(client: &PgPool, code: InvitationCode) -> InviteRet {
    sqlx::query_as!(
        InvitationCode,
        "INSERT INTO invitations(sender, code, send_to, is_used) \
        VALUES ($1, $2, $3, $4) RETURNING *;",
        code.sender.unwrap(),
        code.code,
        code.send_to,
        code.is_used,
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::OtherError("Database inconsistent".to_string()))
}

pub async fn find_invitation_by_user(client: &PgPool, username: &str) -> InviteVecRet {
    Ok(sqlx::query_as!(
        InvitationCode,
        "SELECT * FROM invitations \
        WHERE sender = $1;",
        username,
        )
        .fetch_all(client)
        .await?)
}

pub async fn find_invitation_by_code(client: &PgPool, code: &str) -> InviteVecRet {
    Ok(sqlx::query_as!(
        InvitationCode,
        "SELECT * FROM invitations \
        WHERE code = $1;",
        code,
        )
        .fetch_all(client)
        .await?)
}

pub async fn update_invitation_usage(client: &PgPool, code: &str) -> InviteRet {
    sqlx::query_as!(
        InvitationCode,
        "UPDATE invitations SET is_used = TRUE \
         WHERE code = $1 RETURNING *;",
        code,
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::OtherError("Database inconsistent".to_string()))
}
