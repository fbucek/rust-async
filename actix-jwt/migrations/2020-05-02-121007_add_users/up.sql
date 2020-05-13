-- Your SQL goes here

CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    -- first_name TEXT NOT NULL,
    -- last_name TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    login_session TEXT NOT NULL
);
