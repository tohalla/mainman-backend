ALTER TABLE account_role ALTER COLUMN organisation DROP NOT NULL;
ALTER TABLE organisation DROP COLUMN admin_account;

INSERT INTO account_role (name, rights) VALUES ('administrator', '{"all": true}'::JSONB);
