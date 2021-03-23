-- Add migration script here
DROP TABLE if exists torrent_status;
CREATE TABLE torrent_status(
    tid BIGSERIAL references torrent_info(id),
    uid BIGSERIAL references users(id),
    status INTEGER NOT NULL,
    upload BIGINT NOT NULL DEFAULT 0,
    download BIGINT NOT NULL DEFAULT 0,
    finished BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY(tid, uid)
);
