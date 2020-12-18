CREATE TABLE maintainer_appliance (
  appliance UUID NOT NULL REFERENCES appliance (hash),
  maintainer INTEGER NOT NULL REFERENCES maintainer (id),
  PRIMARY KEY (appliance, maintainer)
);

GRANT SELECT, INSERT, UPDATE, DELETE
  ON TABLE maintainer_appliance TO mainman_client;
