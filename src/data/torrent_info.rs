use super::*;

type TorrentInfoRet = Result<TorrentInfo, Error>;
type TorrentInfoVecRet = Result<Vec<TorrentInfo>, Error>;
type SlimTorrentVecRet = Result<Vec<SlimTorrent>, Error>;

/// A TorrentInfo struct contains
/// 1. title
/// 2. poster: only poster and admin can edit
/// 3. description: full text of post
/// 4. downloaded: total downloads of torrent
/// 5. visible: default is invisible
/// 6. tag: at most 5 tags
/// 7. create_time
/// 8. last_edit
/// 9. last_activity: updated when new comments
#[derive(Serialize, Debug, ToResponse)]
pub struct TorrentInfo {
    pub id: i64,
    pub title: String,
    pub poster: String,
    pub description: Option<String>,
    pub downloaded: i64,
    pub visible: bool,
    pub tag: Option<Vec<String>>,
    pub create_time: DateTime<Utc>,
    pub last_edit: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub stick: bool,
}

/// Slim Version used for list purpose
/// No description and dull information
/// added length from torrent table
#[derive(Serialize, Debug, ToResponse)]
pub struct SlimTorrent {
    pub id: i64,
    pub title: String,
    pub poster: String,
    pub downloaded: i64,
    pub tag: Option<Vec<String>>,
    pub last_activity: DateTime<Utc>,
    pub length: i64,
}

#[derive(Serialize, Debug, ToResponse)]
pub struct JoinedTorrent {
    pub info: TorrentInfo,
    pub torrent: crate::data::torrent::SlimTorrentTable,
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
            // never used
            create_time: Utc::now(),
            last_edit: Utc::now(),
            last_activity: Utc::now(),
            stick: false,
        }
    }
}

/// Add torrent post into database and return the full struct
pub async fn add_torrent_info(client: &sqlx::PgPool, info: TorrentInfo) -> TorrentInfoRet {
    Ok(sqlx::query_as!(
        TorrentInfo,
        "INSERT INTO torrent_info(title, poster, description, create_time, last_edit, last_activity) \
        VALUES ($1, $2, $3, NOW(), NOW(), NOW()) RETURNING *;",
        info.title,
        info.poster,
        info.description
        )
        .fetch_one(client)
        .await?)
}

/// Update the information, will be replaced as a whole
pub async fn update_torrent_info(client: &sqlx::PgPool, id: i64, info: TorrentInfo) -> TorrentInfoRet {
    sqlx::query_as!(
        TorrentInfo,
        "UPDATE torrent_info SET title = $1, description = $2, last_edit = NOW() \
        WHERE id = $3 RETURNING *;",
        info.title,
        info.description,
        id
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

/// add tags as replaced(so front end needs to
/// pass a whole including previous tags)
pub async fn add_tag_for_torrent(client: &sqlx::PgPool, id: i64, tags: &Vec<String>) -> TorrentInfoRet {
    sqlx::query_as!(
        TorrentInfo,
        "UPDATE torrent_info SET tag = $1, last_edit = NOW() \
        WHERE id = $2 RETURNING *;",
        tags,
        id
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

/// Find torrent info by id, return the full structure
pub async fn find_torrent_by_id(client: &sqlx::PgPool, id: i64) -> TorrentInfoRet {
    sqlx::query_as!(
        TorrentInfo,
        "SELECT * FROM torrent_info \
        WHERE id = $1;",
        id
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

/// Find the torrent by poster, return a vector of full struct
///
/// **Is it proper to return slim one?**
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

/// Get counts of total torrents that are not stick
pub async fn query_torrent_counts(client: &sqlx::PgPool) -> CountRet {
    Ok(sqlx::query!(
        "SELECT COUNT(*) FROM torrent_info \
        WHERE visible = TRUE AND stick = FALSE;"
        )
        .fetch_one(client)
        .await?
        .count
        .expect("sql function not right"))
}

/// Find all visible torrents that are not stick
pub async fn find_visible_torrent(client: &sqlx::PgPool, page_offset: i64) -> SlimTorrentVecRet {
    Ok(sqlx::query_as!(
        SlimTorrent,
        "SELECT id, title, poster, downloaded, tag, last_activity, length FROM( \
            SELECT ROW_NUMBER() OVER ( ORDER BY last_activity DESC ) AS RowNum, torrent_info.*, torrent.length \
            FROM torrent_info INNER JOIN torrent ON torrent_info.id = torrent.id \
            WHERE visible = TRUE AND stick = FALSE \
        ) AS RowConstrainedResult \
        WHERE RowNum > $1 AND RowNum <= $1 + 20 \
        ORDER BY RowNum;",
        page_offset
        )
        .fetch_all(client)
        .await?)
}

/// Get counts of torrents definite tags that are not stick
pub async fn query_torrent_counts_by_tag(client: &sqlx::PgPool, tags: &Vec<String>) -> CountRet {
    // due to sqlx not support type cast of postgres
    Ok(sqlx::query_unchecked!(
        "SELECT COUNT(*) FROM torrent_info \
        WHERE visible = TRUE AND ($1::VARCHAR[] <@ tag) AND stick = FALSE;",
        tags
        )
        .fetch_one(client)
        .await?
        .count
        .expect("sql function not right"))
}

/// Find visible torrent with definite tags that are not stick
pub async fn find_visible_torrent_by_tag(client: &sqlx::PgPool, tags: &Vec<String>, page_offset: i64) -> SlimTorrentVecRet {
    // due to sqlx not support type cast of postgres
    Ok(sqlx::query_as_unchecked!(
        SlimTorrent,
        "SELECT id, title, poster, downloaded, tag, last_activity, length FROM( \
            SELECT ROW_NUMBER() OVER ( ORDER BY last_activity DESC ) AS RowNum, torrent_info.*, torrent.length \
            FROM torrent_info INNER JOIN torrent ON torrent_info.id = torrent.id \
            WHERE visible = TRUE AND ($1::VARCHAR[] <@ tag) AND stick = FALSE \
        ) AS RowConstrainedResult \
        WHERE RowNum > $2 AND RowNum <= $2 + 20 \
        ORDER BY RowNum;",
        tags,
        page_offset
        )
        .fetch_all(client)
        .await?)
}

/// Find all stick torrent
pub async fn find_stick_torrent(client: &sqlx::PgPool) -> SlimTorrentVecRet {
    Ok(sqlx::query_as!(
        SlimTorrent,
        "SELECT torrent_info.id, torrent_info.title, torrent_info.poster, \
        torrent_info.downloaded, torrent_info.tag, torrent_info.last_activity, torrent.length \
        FROM torrent_info INNER JOIN torrent ON torrent_info.id = torrent.id \
        WHERE torrent_info.visible = TRUE AND torrent_info.stick = TRUE \
        ORDER BY torrent_info.last_activity DESC;",
        )
        .fetch_all(client)
        .await?)
}

/// make certain torrent visible, accessed by administrator
/// TODO: make it some group action
#[allow(dead_code)]
pub async fn make_torrent_visible(client: &sqlx::PgPool, id: i64) -> TorrentInfoRet {
    sqlx::query_as!(
        TorrentInfo,
        "UPDATE torrent_info SET visible = TRUE \
        WHERE id = $1 RETURNING *;",
        id
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

/// stick some of the torrents
#[allow(dead_code)]
pub async fn make_torrent_stick(client: &sqlx::PgPool, id: i64) -> TorrentInfoRet {
    sqlx::query_as!(
        TorrentInfo,
        "UPDATE torrent_info SET stick = TRUE \
        WHERE id = $1 RETURNING *;",
        id
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}