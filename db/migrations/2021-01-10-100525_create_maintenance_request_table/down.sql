ALTER TABLE maintenance_event
  ADD COLUMN entity uuid REFERENCES entity (hash),
  DROP COLUMN maintenance_request CASCADE;
DROP TABLE maintenance_request;
