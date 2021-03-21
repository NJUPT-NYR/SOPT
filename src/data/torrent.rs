use super::*;
use serde_bytes::ByteBuf;

type TorrentRet = Result<TorrentTable, Error>;

/// a file struct used when parse torrent
#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub path: Vec<String>,
    pub length: i64,
}

/// 1. name: name of torrent
/// 2. pieces: hash pieces of file
/// 3. piece_length: how many pieces there are
/// 4. length: total length of torrent
/// 5. files: file list
/// 6. private: whether torrent is private
/// in our case it is always 1
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

/// 1. info: `Info` struct
/// 2. announce: announce list, in our case it is generated
/// 3. comment: comment by torrent maker
#[derive(Debug, Deserialize, Serialize)]
pub struct Torrent {
    pub info: Info,
    pub announce: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TorrentTable {
    pub id: i64,
    pub name: String,
    pub length: i64,
    pub comment: Option<String>,
    pub files: Vec<String>,
    pub info: Vec<u8>,
    pub infohash: String,
}

pub async fn update_or_add_torrent(client: &sqlx::PgPool, torrent: &TorrentTable, id: i64) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO torrent(id, name, length, comment, files, info, infohash) \
        VALUES($1, $2, $3, $4, $5, $6, $7) \
        ON CONFLICT (id) DO \
        UPDATE SET name = $2, length = $3, comment = $4, files = $5, info = $6, infohash = $7;",
        id,
        torrent.name,
        torrent.length,
        torrent.comment,
        &torrent.files,
        &torrent.info,
        torrent.infohash
        )
        .execute(client)
        .await?;

    Ok(())
}

pub async fn find_torrent_by_id(client: &sqlx::PgPool, id: i64) -> TorrentRet {
    sqlx::query_as!(
        TorrentTable,
        "SELECT * FROM torrent \
        WHERE id = $1;",
        id
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}
