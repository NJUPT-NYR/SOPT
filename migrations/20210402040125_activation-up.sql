-- Add migration script here
DROP TABLE if exists activation;
CREATE TABLE activation(
    id BIGSERIAL PRIMARY KEY REFERENCES users(id),
    code VARCHAR NOT NULL,
    used BOOLEAN NOT NULL DEFAULT false
);