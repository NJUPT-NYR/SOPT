use serde::{Deserialize, Serialize};
use deadpool_postgres::Client;
use tokio_postgres::Row;
use crate::error::Error;
use super::exec_cmd_and_map;

type InviteRet = Result<InvitationCode, Error>;
type InviteVecRet = Result<Vec<InvitationCode>, Error>;

#[derive(Deserialize, Serialize, Debug)]
pub struct InvitationCode {
    pub id: i64,
    pub sender: String,
    pub code: String,
    pub send_to: String,
    pub is_used: bool,
}

impl InvitationCode {
    pub fn new(sender: String, code: String, send_to: String) -> Self {
        InvitationCode {
            id: 1919810,
            sender,
            code,
            send_to,
            is_used: false,
        }
    }
}

fn get_general_ret_invitation(row: &Row) -> InvitationCode {
    InvitationCode {
        id: row.get(0),
        sender: row.get(1),
        code: row.get(2),
        send_to: row.get(3),
        is_used: row.get(4),
    }
}

pub async fn add_invitation_code(client: &Client, code: InvitationCode) -> InviteRet {
    exec_cmd_and_map(
        &client,
        &"INSERT INTO invitations(sender, code, send_to, is_used) \
        VALUES ($1, $2, $3, $4) RETURNING id, sender, code, send_to, is_used;",
        &[
            &code.sender,
            &code.code,
            &code.send_to,
            &code.is_used,
        ],
        get_general_ret_invitation)
        .await?
        .pop()
        .ok_or(Error::OtherError)
}

pub async fn find_invitation_by_user(client: &Client, username: &str) -> InviteVecRet {
    Ok(exec_cmd_and_map(
        &client,
        &"SELECT * FROM invitations \
        WHERE sender = $1;",
        &[
            &username,
        ],
        get_general_ret_invitation)
        .await?)
}

pub async fn find_invitation_by_code(client: &Client, code: &str) -> InviteVecRet {
    Ok(exec_cmd_and_map(
        &client,
        &"SELECT * FROM invitations \
        WHERE code = $1;",
        &[
            &code,
        ],
        get_general_ret_invitation
    ).await?)
}

pub async fn update_invitation_usage(client: &Client, code: &str) -> InviteRet {
    exec_cmd_and_map(
        &client,
        &"UPDATE invitations SET is_used = TRUE \
         WHERE code = $1 RETURNING *;",
        &[
            &code,
        ],
        get_general_ret_invitation)
        .await?
        .pop()
        .ok_or(Error::OtherError)
}
