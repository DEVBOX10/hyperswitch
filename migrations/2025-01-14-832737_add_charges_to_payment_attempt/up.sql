ALTER TABLE payment_attempt
ADD COLUMN charges JSONB
DEFAULT NULL;