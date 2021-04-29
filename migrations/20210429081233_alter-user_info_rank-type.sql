-- Add migration script here
ALTER TABLE user_info DROP CONSTRAINT user_info_rank_fkey;
ALTER TABLE user_info ALTER COLUMN rank TYPE integer USING 1;
ALTER TABLE user_info ADD CONSTRAINT user_info_rank_fkey FOREIGN KEY (rank) REFERENCES rank(id);