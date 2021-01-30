ALTER TABLE plan ADD COLUMN stripe_product TEXT,
  ADD COLUMN stripe_price TEXT;

GRANT UPDATE (stripe_product, stripe_price) ON plan TO mainman_client;
