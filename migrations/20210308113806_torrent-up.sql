-- Add migration script here
DROP TABLE IF EXISTS torrent;
CREATE TABLE torrent(
    id BIGSERIAL PRIMARY KEY REFERENCES torrent_info(id),
    name VARCHAR NOT NULL,
    length BIGINT NOT NULL DEFAULT 0,
    comment VARCHAR,
    files VARCHAR[] NOT NULL,
    info BYTEA NOT NULL,
    infohash VARCHAR NOT NULL
);
