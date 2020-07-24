CREATE TABLE maintainer (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP,
  organisation INTEGER NOT NULL REFERENCES organisation (id) ON DELETE CASCADE,
  account INTEGER REFERENCES account (id),
  details JSONB
);

GRANT USAGE, SELECT ON SEQUENCE maintainer_id_seq TO mainman_client;
GRANT SELECT, INSERT, UPDATE, DELETE ON TABLE maintainer TO mainman_client;
