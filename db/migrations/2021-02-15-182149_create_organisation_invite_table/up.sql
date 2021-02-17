CREATE TABLE organisation_invite (
  uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  organisation INTEGER NOT NULL REFERENCES organisation (id) ON DELETE CASCADE,
  email TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  UNIQUE(organisation, email)
);

GRANT SELECT, INSERT, UPDATE, DELETE ON TABLE organisation_invite TO mainman_client;
