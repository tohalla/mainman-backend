ALTER TABLE organisation_account
  DROP COLUMN id CASCADE,
  ADD PRIMARY KEY (account, organisation);
