-- Add migration script here
DROP TABLE if exists torrent_info;
CREATE TABLE torrent_info(
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    poster VARCHAR(50) NOT NULL REFERENCES users(username),
    description TEXT,
    downloaded BIGINT NOT NULL DEFAULT 0,
    visible BOOLEAN NOT NULL DEFAULT FALSE,
    tag VARCHAR[]
);
