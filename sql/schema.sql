CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(200) UNIQUE NOT NULL,
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(200) NOT NULL,
    passkey VARCHAR(32) UNIQUE NOT NULL,
    UNIQUE (username, email, passkey)
);

-- CREATE TABLE torrents_info (
--     id BIGSERIAL PRIMARY KEY,
--     title VARCHAR(255) NOT NULL,
--     owner BIGSERIAL REFERENCES users(id),
--     description TEXT,
--     downloadable
-- );
--
-- CREATE TABLE torrents (
--     id BIGSERIAL PRIMARY KEY,
--     info_id BIGSERIAL REFERENCES torrents_info(id)
-- );