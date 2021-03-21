-- Add migration script here
DROP TABLE if exists user_info;
CREATE TABLE user_info(
    id BIGSERIAL PRIMARY KEY REFERENCES users(id),
    username VARCHAR(50) UNIQUE NOT NULL REFERENCES users(username),
    registerTime TIMESTAMPTZ NOT NULL,
    lastActivity TIMESTAMPTZ NOT NULL,
    invitor VARCHAR(50) REFERENCES users(username),
    upload BIGINT NOT NULL DEFAULT 0,
    download BIGINT NOT NULL DEFAULT 0,
    money DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    rank VARCHAR NOT NULL REFERENCES rank(name),
    avatar TEXT,
    other JSON,
    privacy INTEGER NOT NULL DEFAULT 0,
    CHECK ( money >= 0.0 )
);