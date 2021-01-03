ALTER TABLE maintenance_event RENAME COLUMN entity TO appliance;

ALTER TABLE maintainer_entity RENAME TO maintainer_appliance;
ALTER TABLE maintainer_appliance RENAME COLUMN entity TO appliance;

ALTER TABLE entity RENAME TO appliance;
