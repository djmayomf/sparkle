-- Create database
CREATE DATABASE sparkle;

-- Switch to the database
USE sparkle;
-- Create base tables
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    interaction_count INTEGER DEFAULT 0
);

CREATE TABLE knowledge_base (
    id SERIAL PRIMARY KEY,
    topic VARCHAR(100) NOT NULL,
    content JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE system_settings (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    setting_key VARCHAR(100) NOT NULL UNIQUE,
    setting_value JSONB NOT NULL,
    setting_updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_accessed TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    access_count INTEGER DEFAULT 1,
    relevance_score FLOAT DEFAULT 1.0,
    decay_rate FLOAT DEFAULT 0.1
);

CREATE TABLE coding_knowledge (
    id SERIAL PRIMARY KEY,
    language VARCHAR(50) NOT NULL,
    concept_type VARCHAR(100) NOT NULL,
    content JSONB NOT NULL,
    examples JSONB,
    difficulty_level INTEGER,
    prerequisites JSONB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_memory_type ON memory_store(memory_type);
CREATE INDEX idx_memory_relevance ON memory_store(relevance_score);
CREATE INDEX idx_coding_language ON coding_knowledge(language);