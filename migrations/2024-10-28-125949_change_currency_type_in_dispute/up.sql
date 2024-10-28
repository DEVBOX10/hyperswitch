-- Your SQL goes here
ALTER TABLE dispute ALTER COLUMN currency TYPE "Currency" USING currency::"Currency";