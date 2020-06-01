CREATE TABLE account (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP,
  first_name VARCHAR(64),
  last_name VARCHAR(64),
  email VARCHAR(256) NOT NULL UNIQUE,
  password BYTEA NOT NULL
);

GRANT USAGE, SELECT ON SEQUENCE account_id_seq TO mainman_client;
GRANT SELECT, INSERT, UPDATE ON TABLE account TO mainman_client;
