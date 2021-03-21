use super::*;

pub async fn add_torrent_info(client: &sqlx::PgPool, title: &str, poster: &str, description: Option<&str>, tags: &[String]) -> TorrentIdRet {
    Ok(sqlx::query_as!(
        TorrentId,
        "INSERT INTO torrent_info(title, poster, description, createTime, lastEdit, tag) \
        VALUES ($1, $2, $3, NOW(), NOW(), $4) RETURNING id, visible;",
        title,
        poster,
        description,
        tags
        )
        .fetch_one(client)
        .await?)
}

pub async fn update_torrent_info(client: &sqlx::PgPool, id: i64, title: &str, description: Option<&str>, tags: &[String]) -> TorrentIdRet {
    sqlx::query_as!(
        TorrentId,
        "UPDATE torrent_info SET title = $1, description = $2, lastEdit = NOW(), tag = $3 \
        WHERE id = $4 RETURNING id, visible;",
        title,
        description,
        tags,
        id
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

pub async fn find_torrent_by_id_mini(client: &sqlx::PgPool, id: i64) -> MiniTorrentRet {
    sqlx::query_as!(
        MiniTorrent,
        "SELECT poster, visible, free, tag FROM torrent_info \
        WHERE id = $1;",
        id
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

pub async fn make_torrent_visible(client: &sqlx::PgPool, ids: &Vec<i64>) -> MiniTorrentVecRet {
    Ok(sqlx::query_as!(
        MiniTorrent,
        "UPDATE torrent_info SET visible = TRUE \
        WHERE id = ANY($1) RETURNING poster, visible, free, tag;",
        ids
        )
        .fetch_all(client)
        .await?)
}

pub async fn find_stick_torrent(client: &sqlx::PgPool) -> SlimTorrentVecRet {
    Ok(sqlx::query_as!(
        SlimTorrent,
        "SELECT torrent_info.id, title, poster, tag, lastEdit, length,\
        free, downloading, uploading, finished \
        FROM torrent_info INNER JOIN torrent ON torrent_info.id = torrent.id \
        WHERE visible = TRUE AND stick = TRUE \
        ORDER BY lastEdit DESC;",
        )
        .fetch_all(client)
        .await?)
}

/// find visible torrent with definite tags that are not stick
pub async fn find_visible_torrent_by_tag(client: &sqlx::PgPool, tags: &Vec<String>, page_offset: i64) -> SlimTorrentVecRet {
    // due to sqlx not support type cast of postgres
    Ok(sqlx::query_as_unchecked!(
        SlimTorrent,
        "SELECT id, title, poster, tag, lastEdit, length, free, downloading, uploading, finished FROM( \
            SELECT ROW_NUMBER() OVER ( ORDER BY lastEdit DESC ) AS RowNum, torrent_info.*, torrent.length \
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

/// find all visible torrents that are not stick
pub async fn find_visible_torrent(client: &sqlx::PgPool, page_offset: i64) -> SlimTorrentVecRet {
    Ok(sqlx::query_as!(
        SlimTorrent,
        "SELECT id, title, poster, tag, lastEdit, length, free, downloading, uploading, finished FROM( \
            SELECT ROW_NUMBER() OVER ( ORDER BY lastEdit DESC ) AS RowNum, torrent_info.*, torrent.length \
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

pub async fn find_invisible_torrent(client: &sqlx::PgPool) -> SlimTorrentVecRet {
    Ok(sqlx::query_as!(
        SlimTorrent,
        "SELECT torrent_info.id, title, poster, tag, lastEdit, length, \
        free, downloading, uploading, finished \
        FROM torrent_info INNER JOIN torrent ON torrent_info.id = torrent.id \
        WHERE visible = FALSE;"
        )
        .fetch_all(client)
        .await?)
}

pub async fn find_torrent_by_poster(client: &sqlx::PgPool, poster: &str) -> SlimTorrentVecRet {
    Ok(sqlx::query_as!(
        SlimTorrent,
        "SELECT torrent_info.id, title, poster, tag, lastEdit, \
        length, free, downloading, uploading, finished \
        FROM torrent_info INNER JOIN torrent ON torrent_info.id = torrent.id \
        WHERE poster = $1",
        poster
        )
        .fetch_all(client)
        .await?)
}

pub async fn find_torrent_by_id(client: &sqlx::PgPool, id: i64) -> FullTorrentRet {
    sqlx::query_as!(
        FullTorrent,
        "SELECT torrent_info.id, title, poster, description, tag, visible, createTime, lastEdit, free, downloading, \
        uploading, finished, length, files, infohash \
        FROM torrent_info INNER JOIN torrent ON torrent_info.id = torrent.id \
        WHERE torrent_info.id = $1;",
        id
        )
        .fetch_all(client)
        .await?
        .pop()
        .ok_or(Error::NotFound)
}

/// get counts of total torrents that are not stick
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

/// get counts of torrents definite tags that are not stick
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

pub async fn make_torrent_stick(client: &sqlx::PgPool, ids: &Vec<i64>) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE torrent_info SET stick = TRUE \
        WHERE id = ANY($1);",
        ids
        )
        .execute(client)
        .await?;

    Ok(())
}

pub async fn make_torrent_free(client: &sqlx::PgPool, ids: &Vec<i64>) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE torrent_info SET free = TRUE \
        WHERE id = ANY($1);",
        ids
        )
        .execute(client)
        .await?;

    Ok(())
}

pub async fn update_torrent_status(client: &sqlx::PgPool, id: i64, downloading: i32, uploading: i32, finished: i64) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE torrent_info SET downloading = downloading + $1, \
        uploading = uploading + $2, finished = finished + $3 \
        WHERE id = $4;",
        downloading,
        uploading,
        finished,
        id
        )
        .execute(client)
        .await?;

    Ok(())
}