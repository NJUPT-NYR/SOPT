-- Add migration script here
DROP TABLE if exists message;
CREATE TABLE message(
    id BIGSERIAL PRIMARY KEY,
    sender VARCHAR(50) NOT NULL REFERENCES users(username),
    receiver VARCHAR(50) NOT NULL REFERENCES users(username),
    title VARCHAR NOT NULL,
    body TEXT,
    read BOOLEAN NOT NULL DEFAULT FALSE,
    visibleSender BOOLEAN NOT NULL DEFAULT TRUE,
    visibleReceiver BOOLEAN NOT NULL DEFAULT TRUE,
    sendTime TIMESTAMPTZ NOT NULL
);