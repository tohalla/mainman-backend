ALTER TABLE plan ALTER COLUMN stripe_price TYPE TEXT USING null;

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
