-- Add migration script here
ALTER TABLE torrent_info ADD stick BOOLEAN NOT NULL DEFAULT false;