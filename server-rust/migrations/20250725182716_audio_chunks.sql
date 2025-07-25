-- Add migration script here
CREATE EXTENSION IF NOT EXISTS vector;
DROP TABLE IF EXISTS audio_chunks;
CREATE TABLE audio_chunks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    roomId UUID REFERENCES rooms(id) NOT NULL,
    transcription TEXT NOT NULL,
    embeddings VECTOR(768) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
);