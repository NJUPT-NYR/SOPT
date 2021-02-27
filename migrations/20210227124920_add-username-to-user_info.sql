-- Add migration script here
ALTER TABLE user_info ADD username VARCHAR(50) UNIQUE NOT NULL DEFAULT '' REFERENCES users(username)