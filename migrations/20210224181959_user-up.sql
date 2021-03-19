-- Add migration script here
DROP TABLE if exists invitations;
DROP TABLE if exists user_info;
DROP TABLE if exists rank;
DROP TABLE IF EXISTS torrent;
DROP TABLE if exists torrent_info;
DROP TABLE if exists users;
CREATE TABLE users (
                       id BIGSERIAL PRIMARY KEY,
                       email VARCHAR(200) UNIQUE NOT NULL,
                       username VARCHAR(50) UNIQUE NOT NULL,
                       password VARCHAR(200) NOT NULL,
                       passkey VARCHAR(32) UNIQUE NOT NULL,
                       UNIQUE (username, email, passkey)
);