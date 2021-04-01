DROP VIEW organisation_overview;
DROP VIEW organisation_maintenance_overview;
DROP VIEW entity_overview;
DROP VIEW maintainer_overview;

ALTER TABLE account ALTER COLUMN id SET DATA TYPE bigint;
ALTER TABLE maintainer
  ALTER COLUMN id SET DATA TYPE bigint,
  ALTER COLUMN organisation SET DATA TYPE bigint,
  ALTER COLUMN account SET DATA TYPE bigint;
ALTER TABLE entity ALTER COLUMN organisation SET DATA TYPE bigint;
ALTER TABLE organisation ALTER COLUMN id SET DATA TYPE bigint;
ALTER TABLE maintenance_event ALTER COLUMN id SET DATA TYPE bigint;
ALTER TABLE maintenance_request ALTER COLUMN created_by SET DATA TYPE bigint;
ALTER TABLE organisation_invite ALTER COLUMN organisation SET DATA TYPE bigint;
ALTER TABLE template ALTER COLUMN organisation SET DATA TYPE bigint;
ALTER TABLE organisation_account
  ALTER COLUMN account SET DATA TYPE bigint,
  ALTER COLUMN organisation SET DATA TYPE bigint,
  ALTER COLUMN account_role SET DATA TYPE bigint;
ALTER TABLE maintenance_task
  ALTER COLUMN maintenance_event SET DATA TYPE bigint,
  ALTER COLUMN maintainer SET DATA TYPE bigint;
ALTER TABLE maintainer_entity
  ALTER COLUMN maintainer SET DATA TYPE bigint,
  ALTER COLUMN organisation SET DATA TYPE bigint;
ALTER TABLE refresh_token ALTER COLUMN account_id SET DATA TYPE bigint;
ALTER TABLE account_role
  ALTER COLUMN id SET DATA TYPE bigint,
  ALTER COLUMN organisation SET DATA TYPE bigint;

CREATE OR REPLACE FUNCTION generate_refresh_token(
  account_id bigint,
  authentication_token text
) RETURNS uuid AS $$
  DECLARE generated_token text;
  BEGIN
    INSERT INTO refresh_token(account_id, token, authentication_token) VALUES
      (account_id, gen_random_uuid(), authentication_token)
    RETURNING (token) INTO generated_token;
    RETURN generated_token;
  END;
$$ LANGUAGE plpgsql;

DROP FUNCTION validate_refresh_token(uuid, text);
CREATE FUNCTION validate_refresh_token(
  token uuid,
  authentication_token text
) RETURNS bigint AS $$
  DECLARE account_id bigint;
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

CREATE OR REPLACE VIEW entity_overview as
  SELECT
    entity.uuid,
    entity.organisation,
    COUNT(DISTINCT maintenance_request.id) FILTER (WHERE maintenance_event.maintenance_request IS NULL) pending_requests,
    COUNT(DISTINCT maintenance_request.id) FILTER (WHERE maintenance_event.maintenance_request IS NOT NULL AND maintenance_event.resolved_at IS NULL) unfinished_requests,
    COUNT(DISTINCT maintenance_request.id) FILTER (WHERE maintenance_event.resolved_at IS NOT NULL) finished_requests
  FROM entity
    LEFT JOIN maintenance_request ON maintenance_request.entity = entity.uuid
    LEFT JOIN maintenance_event ON maintenance_event.maintenance_request = maintenance_request.id
  GROUP BY entity.uuid, entity.organisation;

CREATE OR REPLACE VIEW maintainer_overview as
  SELECT
    maintainer.id,
    maintainer.organisation,
    COUNT(DISTINCT maintenance_task.uuid) FILTER (WHERE maintenance_task.uuid IS NOT NULL AND maintenance_task.is_available = TRUE AND maintenance_task.accepted_at IS NULL) pending_tasks,
    COUNT(DISTINCT maintenance_task.uuid) FILTER (WHERE maintenance_task.uuid IS NOT NULL AND maintenance_task.accepted_at IS NOT NULL AND maintenance_task.resolved_at IS NULL) active_tasks,
    COUNT(DISTINCT maintenance_task.uuid) FILTER (WHERE maintenance_task.uuid IS NOT NULL AND maintenance_task.accepted_at IS NOT NULL AND maintenance_task.resolved_at IS NOT NULL) finished_tasks
  FROM maintainer
    LEFT JOIN maintenance_task ON maintenance_task.maintainer = maintainer.id
  GROUP BY maintainer.id, maintainer.organisation;

CREATE OR REPLACE VIEW organisation_maintenance_overview as
  SELECT
    organisation.id,
    SUM(entity_overview.pending_requests) pending_requests,
    SUM(entity_overview.unfinished_requests) unfinished_requests,
    SUM(entity_overview.finished_requests) finished_requests
  FROM organisation
    LEFT JOIN entity_overview ON entity_overview.organisation = organisation.id
    LEFT JOIN maintainer_overview ON maintainer_overview.organisation = organisation.id
  GROUP BY organisation.id;

CREATE OR REPLACE VIEW organisation_overview as
  SELECT
    organisation.id,
    (SELECT COUNT(*) FROM maintainer WHERE maintainer.organisation = organisation.id) maintainers,
    (SELECT COUNT(*) FROM entity WHERE entity.organisation = organisation.id) entities,
    (SELECT COUNT(*) FROM organisation_account WHERE organisation_account.organisation = organisation.id) accounts
  FROM organisation;

GRANT SELECT ON organisation_overview, organisation_maintenance_overview, entity_overview, maintainer_overview TO mainman_client;
