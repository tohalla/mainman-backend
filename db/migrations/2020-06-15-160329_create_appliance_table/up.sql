CREATE TABLE appliance (
  hash UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP,
  name VARCHAR(256) NOT NULL,
  description TEXT,
  organisation INTEGER NOT NULL REFERENCES organisation (id) ON DELETE CASCADE
);

GRANT SELECT, INSERT, UPDATE, DELETE ON TABLE appliance TO mainman_client;
