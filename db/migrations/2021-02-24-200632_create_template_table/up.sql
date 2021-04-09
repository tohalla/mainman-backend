CREATE TABLE template_type (
  id serial PRIMARY KEY,
  name varchar(64) NOT NULL UNIQUE
);

CREATE TABLE template (
  id bigserial PRIMARY KEY,
  created_at timestamp NOT NULL DEFAULT NOW(),
  updated_at timestamp,
  organisation integer NOT NULL REFERENCES organisation (id) ON DELETE CASCADE,
  name varchar(64),
  content text NOT NULL,
  is_draft bool NOT NULL DEFAULT False,
  template_type integer NOT NULL REFERENCES template_type (id)
);

INSERT INTO template_type (name) VALUES ('maintenance_request'), ('maintenance_report');

GRANT SELECT, INSERT, UPDATE, DELETE
  ON TABLE template TO mainman_client;
GRANT SELECT ON TABLE template_type TO mainman_client;
