ALTER TABLE guess ALTER COLUMN idx DROP NOT NULL;
ALTER TABLE guess RENAME COLUMN idx TO guess_num;