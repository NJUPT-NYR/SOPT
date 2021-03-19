-- Add migration script here
DROP TABLE if exists rank;
CREATE TABLE rank(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    role SMALLINT[] NOT NULL,
    upload BIGINT,
    age BIGINT,
    available BOOLEAN NOT NULL,
    next INT,
    UNIQUE(name)
);