-- This file should undo anything in `up.sql`
ALTER TABLE dispute ALTER COLUMN currency TYPE VARCHAR(255) USING currency::VARCHAR(255);