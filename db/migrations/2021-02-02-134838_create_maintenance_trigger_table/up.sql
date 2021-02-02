CREATE TABLE maintenance_trigger (
  hash UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  created_at timestamp NOT NULL DEFAULT NOW(),
  entity uuid NOT NULL REFERENCES entity (hash)
);

ALTER TABLE maintenance_request
  ADD COLUMN maintenance_trigger UUID REFERENCES maintenance_trigger(hash);

GRANT SELECT, INSERT, UPDATE, DELETE
  ON TABLE maintenance_trigger TO mainman_client;
