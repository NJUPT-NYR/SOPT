-- Add migration script here
ALTER TABLE user_info ADD privacy INTEGER NOT NULL DEFAULT 0;