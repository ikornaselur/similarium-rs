-- Drop the channel.minute column
ALTER TABLE channel DROP COLUMN minute;

-- Revert channel.hour back to bigint
ALTER TABLE channel ALTER COLUMN hour TYPE bigint USING hour::bigint;
