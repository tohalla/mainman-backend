ALTER TABLE maintenance_trigger
  ADD COLUMN template bigint,
  ADD CONSTRAINT maintenance_trigger_template_fkey FOREIGN KEY (template) REFERENCES template (id);
