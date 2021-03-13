-- Add migration script here
ALTER TABLE torrent_info ADD create_time TIMESTAMPTZ NOT NULL default '1999-12-31 23:59:59-07';
ALTER TABLE torrent_info ADD last_edit TIMESTAMPTZ NOT NULL default '1999-12-31 23:59:59-07';
ALTER TABLE torrent_info ADD last_activity TIMESTAMPTZ NOT NULL default '1999-12-31 23:59:59-07';
