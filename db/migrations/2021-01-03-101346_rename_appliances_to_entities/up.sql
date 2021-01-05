ALTER TABLE appliance RENAME TO entity;

ALTER TABLE maintainer_appliance RENAME TO maintainer_entity;
ALTER TABLE maintainer_entity RENAME COLUMN appliance TO entity;

ALTER TABLE maintenance_event RENAME COLUMN appliance TO entity;

ALTER TABLE entity RENAME CONSTRAINT appliance_organisation_fkey TO entity_organisation_fkey;
ALTER TABLE maintenance_event RENAME CONSTRAINT maintenance_event_appliance_fkey TO maintenance_event_entity_fkey;
ALTER TABLE maintainer_entity RENAME CONSTRAINT maintainer_appliance_appliance_fkey TO maintainer_entity_entity_fkey;
ALTER TABLE maintainer_entity RENAME CONSTRAINT maintainer_appliance_maintainer_fkey TO maintainer_entity_maintainer_fkey;
ALTER TABLE entity RENAME CONSTRAINT appliance_pkey TO entity_pkey;
ALTER TABLE maintainer_entity RENAME CONSTRAINT maintainer_appliance_pkey TO maintainer_entity_pkey;
