ALTER TABLE maintainer_entity DROP COLUMN organisation CASCADE;
ALTER TABLE entity DROP CONSTRAINT entity_organisation_key;
ALTER TABLE maintainer DROP CONSTRAINT maintainer_organisation_key;
ALTER TABLE maintainer_entity
  ADD CONSTRAINT maintainer_entity_entity_fkey FOREIGN KEY (entity) REFERENCES entity (uuid),
  ADD CONSTRAINT maintainer_entity_maintainer_fkey FOREIGN KEY (maintainer) REFERENCES maintainer (id);
