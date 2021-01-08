CREATE TABLE plan (
  id serial PRIMARY KEY,
  name text NOT NULL,
  entities integer,
  maintainers integer,
  accounts integer,
  is_public boolean NOT NULL DEFAULT false
);

ALTER TABLE organisation ADD COLUMN plan integer NOT NULL REFERENCES plan(id);

CREATE UNIQUE INDEX name ON plan (LOWER(name));

GRANT SELECT ON TABLE plan TO mainman_client;

INSERT INTO plan (name, entities, maintainers, accounts, is_public) VALUES
  ('Free Plan', 5, 1, 1, true),
  ('Starter Plan', 30, 5, 2, true),
  ('Standard Plan', 100, 25, 5, true);
