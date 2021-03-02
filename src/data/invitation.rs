use serde::Serialize;
use crate::error::Error;
use super::*;
use sopt::*;

type InviteVecRet = Result<Vec<InvitationCode>, Error>;
type SlimInvitationRet = Result<SlimInvitation, Error>;
type SlimInvitationRecRet = Result<Vec<SlimInvitation>, Error>;

/// A invitation code struct contains
/// 1. sender: invitor of this code
/// 2. code: invitation code itself
/// 3. send_to: this is a email address
/// 4. is_used: whether it is used
#[derive(Serialize, Debug, ToResponse)]
pub struct InvitationCode {
    pub id: i64,
    pub sender: Option<String>,
    pub code: String,
    pub send_to: String,
    pub is_used: bool,
}

/// A wrapper for json,
/// remove unnecessary sender and id columns
#[derive(Serialize, Debug, ToResponse)]
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

/// Add invitation into database and return the full `SlimInvitation` struct
/// Return a `SlimInvitation`
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

/// Find all codes sent by one invitor
/// Return a `Vec<SlimInvitation>`
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

/// Find the unique full column by code,
/// useful to check whether the code is valid.
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

/// Called when invitation code is used.
pub async fn update_invitation_usage(client: &sqlx::PgPool, code: &str) -> Result<(), Error> {
    sqlx::query_as!(
        InvitationCode,
        "UPDATE invitations SET is_used = TRUE \
         WHERE code = $1;",
        code,
        )
        .execute(client)
        .await?;

    Ok(())
}
