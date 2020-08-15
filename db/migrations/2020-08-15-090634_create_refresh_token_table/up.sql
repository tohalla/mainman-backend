-- should not give permissions to emotiontracker_client for refresh tokens
CREATE TABLE refresh_token (
  created_at TIMESTAMP DEFAULT NOW(),
  expires_at TIMESTAMP NOT NULL,
  account INTEGER NOT NULL REFERENCES account (id) ON DELETE CASCADE,
  token BYTEA PRIMARY KEY,
  authentication_token BYTEA UNIQUE
);

CREATE OR REPLACE FUNCTION generate_token(length INTEGER)
RETURNS BYTEA AS $$
  SELECT DECODE(
    STRING_AGG(LPAD(TO_HEX(WIDTH_BUCKET(RANDOM(), 0, 1, 256) - 1), 2, '0'), ''),
    'hex'
  ) FROM GENERATE_SERIES(1, $1);
; $$ LANGUAGE SQL VOLATILE;

CREATE OR REPLACE FUNCTION generate_refresh_token(
  account INTEGER,
  expires_at TIMESTAMP DEFAULT NOW() + INTERVAL '7 days'
) RETURNS BYTEA AS $$
  DECLARE generated_token BYTEA;
  BEGIN
    INSERT INTO refresh_token(account, expires_at, token) VALUES
      (account, expires_at, generate_token(512))
    RETURNING (token) INTO generated_token;
    RETURN generated_token;
  END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION is_valid_refresh_token(
  account INTEGER,
  token BYTEA
) RETURNS BOOLEAN AS $$
  DECLARE is_valid BOOLEAN;
  BEGIN
    SELECT EXISTS(
      SELECT 1 FROM refresh_token WHERE refresh_token.account = $1
        AND refresh.token = $2
        AND NOW() < refres_token.expires_at
    ) INTO is_valid;

    -- Tokens should not be used multiple times
    DELETE FROM refresh_token WHERE refresh_token.token = $2;

    RETURN is_valid;
  END;
$$ LANGUAGE plpgsql;

GRANT SELECT, INSERT, DELETE ON refresh_token TO mainman_client;

GRANT EXECUTE ON FUNCTION
  generate_refresh_token(account INTEGER, expires_at TIMESTAMP),
  is_valid_refresh_token(account INTEGER, token BYTEA)
  TO mainman_client;
