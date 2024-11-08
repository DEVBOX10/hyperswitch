-- Your SQL goes here
ALTER TABLE dispute ALTER COLUMN currency TYPE "Currency" USING currency::"Currency"; -- Migration query to be run after deployment before running this query