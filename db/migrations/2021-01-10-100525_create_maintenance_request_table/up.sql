CREATE TABLE maintenance_request (
  id bigserial PRIMARY KEY,
  created_at timestamp NOT NULL DEFAULT NOW(),
  created_by int REFERENCES account (id),
  entity uuid NOT NULL REFERENCES entity (hash),
  description text
);

ALTER TABLE maintenance_event
  DROP COLUMN entity CASCADE,
  ADD COLUMN maintenance_request bigint REFERENCES maintenance_request(id);

GRANT USAGE, SELECT ON SEQUENCE maintenance_request_id_seq TO mainman_client;
GRANT SELECT, INSERT, UPDATE, DELETE
  ON TABLE maintenance_request TO mainman_client;
