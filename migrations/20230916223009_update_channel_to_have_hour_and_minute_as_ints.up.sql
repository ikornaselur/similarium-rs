-- Update Channel.hour to be int
ALTER TABLE channel ALTER COLUMN hour TYPE int USING hour::int;

-- Add Channel.minute as int, default 0 for the migration
ALTER TABLE channel ADD COLUMN minute int;
UPDATE channel SET minute = 0;
ALTER TABLE channel ALTER COLUMN minute SET NOT NULL;
