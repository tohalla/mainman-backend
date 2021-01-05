ALTER TABLE entity RENAME CONSTRAINT entity_organisation_fkey TO appliance_organisation_fkey;
ALTER TABLE maintenance_event RENAME CONSTRAINT maintenance_event_entity_fkey TO maintenance_event_appliance_fkey;
ALTER TABLE maintainer_entity RENAME CONSTRAINT maintainer_entity_entity_fkey TO maintainer_appliance_appliance_fkey;
ALTER TABLE maintainer_entity RENAME CONSTRAINT maintainer_entity_maintainer_fkey TO maintainer_appliance_maintainer_fkey;
ALTER TABLE entity RENAME CONSTRAINT entity_pkey TO appliance_pkey;
ALTER TABLE maintainer_entity RENAME CONSTRAINT maintainer_entity_pkey TO maintainer_appliance_pkey;

ALTER TABLE maintenance_event RENAME COLUMN entity TO appliance;

ALTER TABLE maintainer_entity RENAME TO maintainer_appliance;
ALTER TABLE maintainer_appliance RENAME COLUMN entity TO appliance;

ALTER TABLE entity RENAME TO appliance;
