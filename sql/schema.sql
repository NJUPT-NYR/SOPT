DROP SCHEMA IF EXISTS sopt CASCADE;
CREATE SCHEMA sopt;

CREATE TABLE sopt.users (
                               id  BIGSERIAL PRIMARY KEY,
                               email       VARCHAR(200) NOT NULL,
                               username    VARCHAR(200) UNIQUE NOT NULL,
                               password    VARCHAR(200) NOT NULL,
                               UNIQUE (username)
);