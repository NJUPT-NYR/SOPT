-- Add migration script here
DROP TABLE if exists tag;
CREATE TABLE tag(
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR UNIQUE NOT NULL,
    amount INTEGER NOT NULL DEFAULT 0,
    UNIQUE(name)
);