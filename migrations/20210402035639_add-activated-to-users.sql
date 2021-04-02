-- Add migration script here
ALTER TABLE users ADD activated BOOLEAN NOT NULL DEFAULT false;