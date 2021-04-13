ALTER TABLE maintenance_event ADD COLUMN entity UUID NOT NULL,
  ADD CONSTRAINT maintenance_event_entity_fkey FOREIGN KEY (entity) REFERENCES entity (uuid);
