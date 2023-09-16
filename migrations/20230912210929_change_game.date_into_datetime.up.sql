-- Change game.date from string to datetime with timezone
ALTER TABLE
  game
ALTER COLUMN
  date TYPE TIMESTAMP WITH TIME ZONE USING date::TIMESTAMP WITH TIME ZONE;
