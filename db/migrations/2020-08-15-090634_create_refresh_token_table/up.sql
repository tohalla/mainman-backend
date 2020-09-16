CREATE EXTENSION "pgcrypto";

CREATE TABLE refresh_token (
  created_at TIMESTAMP DEFAULT NOW(),
  account INTEGER NOT NULL REFERENCES account (id) ON DELETE CASCADE,
  token UUID PRIMARY KEY,
  authentication_token TEXT UNIQUE
);

CREATE OR REPLACE FUNCTION generate_refresh_token(
  account INTEGER,
  authentication_token TEXT
) RETURNS TEXT AS $$
  DECLARE generated_token TEXT;
  BEGIN
    INSERT INTO refresh_token(account, token, authentication_token) VALUES
      (account, gen_random_uuid(), authentication_token)
    RETURNING (token) INTO generated_token;
    RETURN generated_token;
  END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION is_valid_refresh_token(
  account INTEGER,
  token UUID,
  authentication_token TEXT
) RETURNS BOOLEAN AS $$
  DECLARE is_valid BOOLEAN;
  BEGIN
    SELECT EXISTS(
      SELECT 1 FROM refresh_token WHERE refresh_token.account = $1
        AND refresh_token.token = $2
        AND refresh_token.authentication_token = $3
        AND NOW() < refresh_token.created_at + INTERVAL '7 days'
    ) INTO is_valid;

    -- Tokens should not be used multiple times
    DELETE FROM refresh_token WHERE refresh_token.token = $2;

    RETURN is_valid;
  END;
$$ LANGUAGE plpgsql;

GRANT SELECT, INSERT, DELETE ON refresh_token TO mainman_client;

GRANT EXECUTE ON FUNCTION
  generate_refresh_token(account INTEGER, authentication_token TEXT),
  is_valid_refresh_token(
    account INTEGER,
    token UUID,
    authentication_token TEXT
  )
  TO mainman_client;
