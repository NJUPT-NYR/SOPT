use serde::{Deserialize, Serialize};
use crate::error::Error;

type InviteRet = Result<InvitationCode, Error>;
type InviteVecRet = Result<Vec<InvitationCode>, Error>;
type SlimInvitationRet = Result<SlimInvitation, Error>;
type SlimInvitationRecRet = Result<Vec<SlimInvitation>, Error>;

#[derive(Deserialize, Serialize, Debug)]
pub struct InvitationCode {
    pub id: i64,
    pub sender: Option<String>,
    pub code: String,
    pub send_to: String,
    pub is_used: bool,
}

// A wrapper for json
#[derive(Deserialize, Serialize, Debug)]
pub struct SlimInvitation {
    pub code: String,
    #[serde(rename="sendTo")]
    pub send_to: String,
    #[serde(rename="isUsed")]
    pub is_used: bool,
}

impl SlimInvitation {
    fn from(full: &InvitationCode) -> Self {
        SlimInvitation {
            // fuck u borrow checker
            // so much copy
            code: full.code.to_string(),
            send_to: full.send_to.to_string(),
            is_used: full.is_used,
        }
    }
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

pub async fn add_invitation_code(client: &sqlx::PgPool, code: InvitationCode) -> SlimInvitationRet {
    let ret: InvitationCode = sqlx::query_as!(
        InvitationCode,
        "INSERT INTO invitations(sender, code, send_to, is_used) \
        VALUES ($1, $2, $3, $4) RETURNING *;",
        code.sender.unwrap(),
        code.code,
        code.send_to,
        code.is_used,
        )
        .fetch_one(client)
        .await?;

    Ok(SlimInvitation::from(&ret))
}

pub async fn find_invitation_by_user(client: &sqlx::PgPool, username: &str) -> SlimInvitationRecRet {
    let vec: Vec<InvitationCode> = sqlx::query_as!(
        InvitationCode,
        "SELECT * FROM invitations \
        WHERE sender = $1;",
        username,
        )
        .fetch_all(client)
        .await?;

    Ok(vec.iter().map(|row| SlimInvitation::from(row)).collect())
}

pub async fn find_invitation_by_code(client: &sqlx::PgPool, code: &str) -> InviteVecRet {
    Ok(sqlx::query_as!(
        InvitationCode,
        "SELECT * FROM invitations \
        WHERE code = $1;",
        code,
        )
        .fetch_all(client)
        .await?)
}

pub async fn update_invitation_usage(client: &sqlx::PgPool, code: &str) -> InviteRet {
    Ok(sqlx::query_as!(
        InvitationCode,
        "UPDATE invitations SET is_used = TRUE \
         WHERE code = $1 RETURNING *;",
        code,
        )
        .fetch_one(client)
        .await?)
}
