-- Add migration script here
DROP TABLE IF EXISTS torrent;

CREATE TABLE torrent(
    id BIGSERIAL PRIMARY KEY REFERENCES torrent_info(id),
    name TEXT NOT NULL,
    length BIGINT NOT NULL DEFAULT 0,
    comment TEXT,
    files TEXT[] NOT NULL,
    info BYTEA NOT NULL
);
