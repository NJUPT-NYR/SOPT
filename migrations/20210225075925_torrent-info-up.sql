-- Add migration script here
DROP TABLE if exists torrent_info;
CREATE TABLE torrent_info(
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    poster VARCHAR(50) NOT NULL REFERENCES users(username),
    description TEXT,
    visible BOOLEAN NOT NULL DEFAULT FALSE,
    tag VARCHAR[],
    createTime TIMESTAMPTZ NOT NULL,
    lastEdit TIMESTAMPTZ NOT NULL,
    stick BOOLEAN NOT NULL DEFAULT FALSE,
    free BOOLEAN NOT NULL DEFAULT FALSE,
    downloading INTEGER NOT NULL DEFAULT 0,
    uploading INTEGER NOT NULL DEFAULT 0,
    finished BIGINT NOT NULL DEFAULT 0
);
