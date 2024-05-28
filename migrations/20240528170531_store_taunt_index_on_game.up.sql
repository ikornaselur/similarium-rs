-- Add taunt_index to Game
ALTER TABLE game ADD COLUMN taunt_index bigint;
UPDATE game SET taunt_index = 0;
ALTER TABLE game ALTER COLUMN taunt_index SET NOT NULL;
