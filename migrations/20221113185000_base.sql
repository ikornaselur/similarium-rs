CREATE TABLE similarity_range (
	word TEXT NOT NULL, 
	top FLOAT NOT NULL, 
	top10 FLOAT NOT NULL, 
	rest FLOAT NOT NULL, 
	PRIMARY KEY (word)
);
CREATE TABLE user (
	id TEXT NOT NULL, 
	profile_photo TEXT NOT NULL, 
	username TEXT NOT NULL, 
	PRIMARY KEY (id)
);
CREATE TABLE word2vec (
	word TEXT NOT NULL, 
	vec BLOB NOT NULL, 
	PRIMARY KEY (word)
);
CREATE TABLE nearby (
	word TEXT NOT NULL, 
	neighbor TEXT NOT NULL, 
	similarity FLOAT NOT NULL, 
	percentile INTEGER NOT NULL, 
	PRIMARY KEY (word, neighbor), 
	FOREIGN KEY(word) REFERENCES word2vec (word), 
	FOREIGN KEY(neighbor) REFERENCES word2vec (word)
);
CREATE TABLE IF NOT EXISTS "game" (
	id INTEGER NOT NULL, 
	channel_id TEXT NOT NULL, 
	thread_ts TEXT NOT NULL, 
	puzzle_number INTEGER NOT NULL, 
	date TEXT NOT NULL, 
	active BOOLEAN NOT NULL, 
	secret TEXT NOT NULL, 
	PRIMARY KEY (id), 
	CONSTRAINT channel_id FOREIGN KEY(channel_id) REFERENCES channel (id)
);
CREATE INDEX channel_thread_idx ON game (channel_id, thread_ts);
CREATE TABLE IF NOT EXISTS "guess" (
	id INTEGER NOT NULL, 
	game_id INTEGER NOT NULL, 
	updated INTEGER NOT NULL, 
	user_id TEXT NOT NULL, 
	word TEXT NOT NULL, 
	percentile INTEGER NOT NULL, 
	similarity FLOAT NOT NULL, 
	idx INTEGER NOT NULL, 
	latest_guess_user_id TEXT NOT NULL, 
	PRIMARY KEY (id), 
	CONSTRAINT latest_guess_user_id FOREIGN KEY(latest_guess_user_id) REFERENCES user (id), 
	FOREIGN KEY(game_id) REFERENCES game (id), 
	FOREIGN KEY(user_id) REFERENCES user (id)
);
CREATE TABLE IF NOT EXISTS "channel" (
	id TEXT NOT NULL, 
	team_id TEXT NOT NULL, 
	hour INTEGER NOT NULL, 
	active BOOLEAN NOT NULL, 
	PRIMARY KEY (id)
);
CREATE TABLE game_user_winner_association (
	game_id INTEGER NOT NULL, 
	user_id TEXT NOT NULL, 
	guess_idx INTEGER NOT NULL, 
	PRIMARY KEY (game_id, user_id), 
	FOREIGN KEY(game_id) REFERENCES game (id), 
	FOREIGN KEY(user_id) REFERENCES user (id)
);
