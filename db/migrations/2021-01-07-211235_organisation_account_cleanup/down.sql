ALTER TABLE organisation_account
  DROP CONSTRAINT organisation_account_pkey,
  ADD COLUMN id serial PRIMARY KEY;
