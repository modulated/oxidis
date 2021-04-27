-- Add migration script here
CREATE TABLE subscriptions (
    id INTEGER PRIMARY KEY NOT NULL,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at TEXT NOT NULL
);