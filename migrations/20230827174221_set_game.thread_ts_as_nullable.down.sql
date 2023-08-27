UPDATE game SET thread_ts = '' WHERE thread_ts IS NULL;
ALTER TABLE game ALTER COLUMN thread_ts SET NOT NULL;
