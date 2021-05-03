-- Add migration script here
INSERT INTO rank(id, name, role, upload, age)
VALUES(1, 'User', '{0 , 2}', 0, 0) ON CONFLICT(id) DO NOTHING;