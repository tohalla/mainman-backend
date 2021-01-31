CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE card (
  id TEXT PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  account INTEGER NOT NULL REFERENCES account (id) ON DELETE CASCADE
);

GRANT SELECT, INSERT, UPDATE, DELETE
  ON TABLE card TO mainman_client;
