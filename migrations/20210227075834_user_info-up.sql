-- Add migration script here
DROP TABLE if exists user_info;
CREATE TABLE user_info(
    id BIGSERIAL PRIMARY KEY REFERENCES users(id),
    register_time TIMESTAMPTZ NOT NULL,
    last_activity TIMESTAMPTZ NOT NULL,
    invitor VARCHAR(50) REFERENCES users(username),
    upload BIGINT NOT NULL DEFAULT 0,
    download BIGINT NOT NULL DEFAULT 0,
    money DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    rank INTEGER NOT NULL DEFAULT 0,
    avatar TEXT,
    other JSON
);