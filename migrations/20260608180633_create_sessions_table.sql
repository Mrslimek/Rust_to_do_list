-- Add migration script here
CREATE TABLE IF NOT EXISTS sessions (
    id BLOB PRIMARY KEY NOT NULL,
    data BLOB NOT NULL,
    expiry_date INTEGER NOT NULL
);
