ALTER TABLE entity ADD COLUMN maintenance_report_template bigint REFERENCES template (id);
