ALTER TABLE entity ADD CONSTRAINT entity_organisation_key UNIQUE (hash, organisation);
ALTER TABLE maintainer ADD CONSTRAINT maintainer_organisation_key UNIQUE (id, organisation);
ALTER TABLE maintainer_entity ADD COLUMN organisation integer NOT NULL,
  DROP CONSTRAINT maintainer_entity_entity_fkey,
  DROP CONSTRAINT maintainer_entity_maintainer_fkey,
  ADD CONSTRAINT maintainer_entity_entity_fkey FOREIGN KEY (entity, organisation) REFERENCES entity (hash, organisation),
  ADD CONSTRAINT maintainer_entity_maintainer_fkey FOREIGN KEY (maintainer, organisation) REFERENCES maintainer (id, organisation);
