DELETE FROM account_role WHERE organisation IS NULL;

ALTER TABLE account_role ALTER COLUMN organisation SET NOT NULL;
ALTER TABLE organisation ADD COLUMN admin_account INTEGER NOT NULL REFERENCES account (id);
