CREATE TABLE account (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP,
  first_name VARCHAR(64),
  last_name VARCHAR(64),
  email VARCHAR(256) NOT NULL,
  password BYTEA NOT NULL
);

CREATE UNIQUE INDEX key_email ON account (LOWER(email));

GRANT USAGE, SELECT ON SEQUENCE account_id_seq TO mainman_client;
GRANT SELECT, INSERT, UPDATE ON TABLE account TO mainman_client;
