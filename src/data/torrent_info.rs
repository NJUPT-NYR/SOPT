use serde::{Deserialize, Serialize};
use crate::error::Error;

type TorrentInfoRet = Result<TorrentInfo, Error>;
type TorrentInfoVecRet = Result<Vec<TorrentInfo>, Error>;
type SlimTorrentVecRet = Result<Vec<SlimTorrent>, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub struct TorrentInfo {
    pub id: i64,
    pub title: String,
    pub poster: String,
    pub description: Option<String>,
    pub downloaded: i64,
    pub visible: bool,
    pub tag: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SlimTorrent {
    pub id: i64,
    pub title: String,
    pub poster: String,
    pub downloaded: i64,
    pub tag: Option<Vec<String>>,
}

impl TorrentInfo {
    pub fn new(title: String, poster: String, description: Option<String>) -> Self {
        TorrentInfo {
            id: 114,
            title,
            poster,
            description,
            downloaded: 0,
            visible: true,
            tag: None,
        }
    }
}

pub async fn add_torrent_info(client: &sqlx::PgPool, info: TorrentInfo) -> TorrentInfoRet {
    let desc = info.description.unwrap_or("".to_string());

    Ok(sqlx::query_as!(
        TorrentInfo,
        "INSERT INTO torrent_info(title, poster, description) \
        VALUES ($1, $2, $3) RETURNING *;",
        info.title,
        info.poster,
        desc
        )
        .fetch_one(client)
        .await?)
}

pub async fn update_torrent_info(client: &sqlx::PgPool, id: i64, info: TorrentInfo) -> TorrentInfoRet {
    let desc = info.description.unwrap_or("".to_string());

    Ok(sqlx::query_as!(
        TorrentInfo,
        "UPDATE torrent_info SET title = $1, description = $2 \
        WHERE id = $3 RETURNING *;",
        info.title,
        desc,
        id
        )
        .fetch_one(client)
        .await?)
}

pub async fn add_tag_for_torrent(client: &sqlx::PgPool, id: i64, tags: &Vec<String>) -> TorrentInfoRet {
    Ok(sqlx::query_as!(
        TorrentInfo,
        "UPDATE torrent_info SET tag = tag || $1 \
        WHERE id = $2 RETURNING *;",
        tags,
        id
        )
        .fetch_one(client)
        .await?)
}

pub async fn find_torrent_by_id(client: &sqlx::PgPool, id: i64) -> TorrentInfoRet {
    Ok(sqlx::query_as!(
        TorrentInfo,
        "SELECT * FROM torrent_info \
        WHERE id = $1;",
        id
        )
        .fetch_one(client)
        .await?)
}

pub async fn find_torrent_by_poster(client: &sqlx::PgPool, poster: String) -> TorrentInfoVecRet {
    Ok(sqlx::query_as!(
        TorrentInfo,
        "SELECT * FROM torrent_info \
        WHERE poster = $1",
        poster
        )
        .fetch_all(client)
        .await?)
}

pub async fn find_visible_torrent(client: &sqlx::PgPool) -> SlimTorrentVecRet {
    Ok(sqlx::query_as!(
        SlimTorrent,
        "SELECT id, title, poster, downloaded, tag FROM torrent_info \
        WHERE visible = TRUE;"
        )
        .fetch_all(client)
        .await?)
}

pub async fn find_visible_torrent_by_tag(client: &sqlx::PgPool, tag: &str) -> SlimTorrentVecRet {
    Ok(sqlx::query_as!(
        SlimTorrent,
        "SELECT id, title, poster, downloaded, tag FROM torrent_info \
        WHERE visible = TRUE AND $1 = ANY(tag);",
        tag
        )
        .fetch_all(client)
        .await?)
}

pub async fn make_torrent_visible(client: &sqlx::PgPool, id: i64) -> TorrentInfoRet {
    Ok(sqlx::query_as!(
        TorrentInfo,
        "UPDATE torrent_info SET visible = TRUE \
        WHERE id = $1 RETURNING *;",
        id
        )
        .fetch_one(client)
        .await?)
}
