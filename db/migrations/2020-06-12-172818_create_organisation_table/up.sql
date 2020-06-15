CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE organisation (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP,
  name VARCHAR(64) NOT NULL,
  organisation_identifier VARCHAR(128),
  locale VARCHAR(10) NOT NULL,
  admin_account INTEGER NOT NULL REFERENCES account (id)
);

CREATE TABLE account_role (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP,
  organisation INTEGER NOT NULL REFERENCES organisation (id) ON DELETE SET NULL,
  name VARCHAR(64) NOT NULL,
  rights JSONB NOT NULL DEFAULT '{}'::JSONB
);

CREATE TABLE organisation_account (
  uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  account INTEGER NOT NULL REFERENCES account (id) ON DELETE CASCADE,
  organisation INTEGER NOT NULL REFERENCES organisation (id) ON DELETE CASCADE,
  account_role INTEGER NOT NULL REFERENCES account_role (id) ON DELETE SET NULL
);

GRANT USAGE, SELECT
  ON SEQUENCE organisation_id_seq, account_role_id_seq TO mainman_client;
GRANT SELECT, INSERT, UPDATE, DELETE
  ON TABLE organisation, account_role, organisation_account TO mainman_client;
