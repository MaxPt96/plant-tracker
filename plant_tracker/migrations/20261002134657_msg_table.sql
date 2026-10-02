-- Active: 1781346659493@@127.0.0.1@5432@plants
-- Add migration script here
DROP TABLE IF EXISTS message_id;

CREATE TABLE message_id (
    user_id BIGINT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    msg_id BIGINT NOT NULL
);