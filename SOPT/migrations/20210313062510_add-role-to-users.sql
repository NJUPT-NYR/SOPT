-- Add migration script here
ALTER TABLE users ADD role BIGINT NOT NULL DEFAULT 1;