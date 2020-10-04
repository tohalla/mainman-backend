CREATE EXTENSION "pgcrypto";

CREATE TABLE refresh_token (
  created_at TIMESTAMP DEFAULT NOW(),
  account_id INTEGER NOT NULL REFERENCES account (id) ON DELETE CASCADE,
  token UUID PRIMARY KEY,
  authentication_token TEXT UNIQUE
);

CREATE OR REPLACE FUNCTION generate_refresh_token(
  account_id INTEGER,
  authentication_token TEXT
) RETURNS UUID AS $$
  DECLARE generated_token TEXT;
  BEGIN
    INSERT INTO refresh_token(account_id, token, authentication_token) VALUES
      (account_id, gen_random_uuid(), authentication_token)
    RETURNING (token) INTO generated_token;
    RETURN generated_token;
  END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION validate_refresh_token(
  token UUID,
  authentication_token TEXT
) RETURNS INTEGER AS $$
  DECLARE account_id INTEGER;
  BEGIN
    SELECT refresh_token.account_id INTO account_id FROM refresh_token
      WHERE refresh_token.token = validate_refresh_token.token
        AND refresh_token.authentication_token = validate_refresh_token.authentication_token
        AND NOW() < refresh_token.created_at + INTERVAL '7 days';

    -- Tokens should not be used multiple times
    DELETE FROM refresh_token
      WHERE refresh_token.token = validate_refresh_token.token;

    RETURN account_id;
  END;
$$ LANGUAGE plpgsql;

GRANT SELECT, INSERT, DELETE ON refresh_token TO mainman_client;

GRANT EXECUTE ON FUNCTION
  generate_refresh_token(account_id INTEGER, authentication_token TEXT),
  validate_refresh_token(
    token UUID,
    authentication_token TEXT
  )
  TO mainman_client;
