-- Add migration script here
ALTER TABLE rank ALTER COLUMN upload SET NOT NULL;
ALTER TABLE rank ALTER COLUMN age SET NOT NULL;