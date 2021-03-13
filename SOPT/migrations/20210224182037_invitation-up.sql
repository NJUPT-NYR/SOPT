-- Add migration script here
DROP TABLE if exists invitations;
CREATE TABLE invitations (
                             id BIGSERIAL PRIMARY KEY,
                             sender VARCHAR(50) REFERENCES users(username),
                             code VARCHAR(200) UNIQUE NOT NULL,
                            -- email address
                             send_to VARCHAR(200) NOT NULL,
                             is_used boolean NOT NULL,
                             UNIQUE (code)
);