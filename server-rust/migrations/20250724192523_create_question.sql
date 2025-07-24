-- Add migration script here
-- Add migration script here
CREATE EXTENSION IF NOT EXISTS vector;
DROP TABLE IF EXISTS questions;
CREATE TABLE questions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    roomId UUID REFERENCES rooms(id),
    question TEXT NOT NULL,
    answer TEXT,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
);