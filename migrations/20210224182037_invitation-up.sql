-- Add migration script here
DROP TABLE if exists invitations;
CREATE TABLE invitations (
    sender VARCHAR(50) REFERENCES users(username),
    code VARCHAR UNIQUE NOT NULL,
    address VARCHAR NOT NULL,
    usage boolean NOT NULL DEFAULT FALSE,
    UNIQUE (code)
);