-- Add migration script here
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);

-- One user has many todos: the FK lives on the "many" side (todos).
ALTER TABLE todos
    ADD COLUMN user_id INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL;
