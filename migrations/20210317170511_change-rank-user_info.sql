-- Add migration script here
ALTER TABLE user_info ALTER COLUMN rank TYPE VARCHAR;
ALTER TABLE user_info ADD CONSTRAINT rank_ref FOREIGN KEY (rank) REFERENCES rank(name);