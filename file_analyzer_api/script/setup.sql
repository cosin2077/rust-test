CREATE DATABASE file_analyzer;
\c file_analyzer
CREATE TABLE file_stats (
  id SERIAL PRIMARY KEY,
  path TEXT NOT NULL,
  word_count INTEGER NOT NULL,
  line_count INTEGER NOT NULL,
  char_count INTEGER NOT NULL,
  unique_words INTEGER NOT NULL,
  word_frequency JSONB NOT NULL,
  analyzed_at TIMESTAMP WITH TIME ZONE NOT NULL
);