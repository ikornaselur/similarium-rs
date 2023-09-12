-- Change game.date from datetime to string
ALTER TABLE
  game
ALTER COLUMN
  date TYPE text USING date::text;

