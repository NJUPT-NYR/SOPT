-- Add migration script here
DROP TABLE if exists invitations;
DROP TABLE if exists user_info;
DROP TABLE if exists rank;
DROP TABLE IF EXISTS torrent;
DROP TABLE if exists torrent_info;
DROP TABLE if exists users;
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR UNIQUE NOT NULL,
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR NOT NULL,
    passkey VARCHAR UNIQUE NOT NULL,
    role BIGINT NOT NULL DEFAULT 1,
    UNIQUE (username, email, passkey)
);