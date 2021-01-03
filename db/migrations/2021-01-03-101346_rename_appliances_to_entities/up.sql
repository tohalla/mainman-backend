ALTER TABLE appliance RENAME TO entity;

ALTER TABLE maintainer_appliance RENAME TO maintainer_entity;
ALTER TABLE maintainer_entity RENAME COLUMN appliance TO entity;

ALTER TABLE maintenance_event RENAME COLUMN appliance TO entity;
