-- Add migration script here
DROP TABLE if exists tag;
CREATE TABLE tag(
    name VARCHAR UNIQUE NOT NULL,
    amount INTEGER NOT NULL DEFAULT 0,
    UNIQUE(name)
);