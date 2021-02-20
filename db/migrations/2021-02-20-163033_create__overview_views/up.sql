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
