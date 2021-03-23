use super::*;

pub async fn find_status_by_tid_uid(client: &sqlx::PgPool, tid: i64, uid: i64) -> TorrentStatusVecRet {
    Ok(sqlx::query_as!(
        TorrentStatus,
        "SELECT * FROM torrent_status \
        WHERE tid = $1 AND uid = $2;",
        tid,
        uid
        )
        .fetch_all(client)
        .await?)
}

pub async fn find_downloading_torrent(client: &sqlx::PgPool, uid: i64) -> PersonalTorrentVecRet {
    Ok(sqlx::query_as!(
        PersonalTorrent,
        "SELECT torrent_info.id, title, length, torrent_status.upload, torrent_status.download \
        FROM torrent_status INNER JOIN torrent ON torrent_status.tid = torrent.id INNER JOIN torrent_info ON \
        torrent.id = torrent_info.id \
        WHERE status = 0 AND uid = $1;",
        uid
        )
        .fetch_all(client)
        .await?)
}

pub async fn find_uploading_torrent(client: &sqlx::PgPool, uid: i64) -> PersonalTorrentVecRet {
    Ok(sqlx::query_as!(
        PersonalTorrent,
        "SELECT torrent_info.id, title, length, torrent_status.upload, torrent_status.download \
        FROM torrent_status INNER JOIN torrent ON torrent_status.tid = torrent.id INNER JOIN torrent_info ON \
        torrent.id = torrent_info.id \
        WHERE status = 1 AND uid = $1;",
        uid
        )
        .fetch_all(client)
        .await?)
}

pub async fn find_finished_torrent(client: &sqlx::PgPool, uid: i64) -> PersonalTorrentVecRet {
    Ok(sqlx::query_as!(
        PersonalTorrent,
        "SELECT torrent_info.id, title, length, torrent_status.upload, torrent_status.download \
        FROM torrent_status INNER JOIN torrent ON torrent_status.tid = torrent.id INNER JOIN torrent_info ON \
        torrent.id = torrent_info.id \
        WHERE torrent_status.finished = TRUE AND uid = $1;",
        uid
        )
        .fetch_all(client)
        .await?)
}

pub async fn find_unfinished_torrent(client: &sqlx::PgPool, uid: i64) -> PersonalTorrentVecRet {
    Ok(sqlx::query_as!(
        PersonalTorrent,
        "SELECT torrent_info.id, title, length, torrent_status.upload, torrent_status.download \
        FROM torrent_status INNER JOIN torrent ON torrent_status.tid = torrent.id INNER JOIN torrent_info ON \
        torrent.id = torrent_info.id \
        WHERE torrent_status.finished = FALSE AND status = 2 AND uid = $1;",
        uid
        )
        .fetch_all(client)
        .await?)
}

pub async fn update_or_add_status(client: &sqlx::PgPool, tid: i64, uid: i64, status: i32, upload: i64, download: i64) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO torrent_status(tid, uid, status, upload, download) \
        VALUES($1, $2, $3, $4, $5) ON CONFLICT (tid, uid) DO \
        UPDATE SET status = $3, upload = torrent_status.upload + $4, download = torrent_status.download + $5;",
        tid,
        uid,
        status,
        upload,
        download
        )
        .execute(client)
        .await?;

    Ok(())
}

pub async fn update_finished_by_tid_uid(client: &sqlx::PgPool, tid: i64, uid: i64) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE torrent_status SET finished = TRUE \
        WHERE tid = $1 AND uid = $2;",
        tid,
        uid
        )
        .execute(client)
        .await?;

    Ok(())
}
