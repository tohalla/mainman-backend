CREATE TABLE maintenance_event (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP,
  resolved_at TIMESTAMP,
  appliance UUID NOT NULL REFERENCES appliance (uuid),
  description TEXT
);

CREATE TABLE maintenance_task (
  uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP,
  accepted_at TIMESTAMP,
  resolved_at TIMESTAMP,
  maintenance_event INTEGER NOT NULL REFERENCES maintenance_event (id),
  maintainer INTEGER NOT NULL REFERENCES maintainer (id),
  is_available BOOLEAN NOT NULL DEFAULT true
);

GRANT USAGE, SELECT ON SEQUENCE maintenance_event_id_seq TO mainman_client;
GRANT SELECT, INSERT, UPDATE, DELETE
  ON TABLE maintenance_event, maintenance_task TO mainman_client;
