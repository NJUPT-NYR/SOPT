use serde::Serialize;
use serde_bytes::ByteBuf;
use crate::error::Error;
use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub path: Vec<String>,
    pub length: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    // much more faster to deserialize
    pub pieces: ByteBuf,
    #[serde(rename="piece length")]
    pub piece_length: i64,
    pub length: Option<i64>,
    pub files: Option<Vec<File>>,
    pub private: Option<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Torrent {
    pub info: Info,
    pub announce: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TorrentTable {
    pub id: i64,
    pub name: String,
    pub length: i64,
    pub comment: Option<String>,
    pub files: Vec<String>,
    pub info: Vec<u8>,
}

pub async fn update_or_add_torrent(client: &sqlx::PgPool, torrent: &TorrentTable, id: i64) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO torrent(id, name, length, comment, files, info) \
        VALUES($1, $2, $3, $4, $5, $6) \
        ON CONFLICT (id) DO \
        UPDATE SET name = $2, length = $3, comment = $4, files = $5, info = $6;",
        id,
        torrent.name,
        torrent.length,
        torrent.comment,
        &torrent.files,
        &torrent.info
        )
        .execute(client)
        .await?;

    Ok(())
}