CREATE TABLE
channel (
    id text NOT NULL,
    team_id text NOT NULL,
    hour bigint NOT NULL,
    active boolean NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE
"user" (
    id text NOT NULL,
    profile_photo text NOT NULL,
    username text NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE
word2vec (
    word text NOT NULL,
    vec bytea NOT NULL,
    PRIMARY KEY (word)
);

CREATE TABLE
game (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,
    channel_id text NOT NULL,
    thread_ts text NOT NULL,
    puzzle_number bigint NOT NULL,
    date text NOT NULL,
    active boolean NOT NULL,
    secret text NOT NULL,
    hint text,
    PRIMARY KEY (id),
    FOREIGN KEY (channel_id) REFERENCES channel (id)
);

CREATE INDEX channel_thread_idx ON game USING btree (
    channel_id, thread_ts
);

CREATE TABLE
game_user_hint_association (
    game_id uuid NOT NULL,
    user_id text NOT NULL,
    created bigint NOT NULL,
    guess_idx integer NOT NULL,
    PRIMARY KEY (game_id, user_id),
    FOREIGN KEY (game_id) REFERENCES game (id),
    FOREIGN KEY (user_id) REFERENCES "user" (id)
);

CREATE TABLE
game_user_winner_association (
    game_id uuid NOT NULL,
    user_id text NOT NULL,
    guess_idx bigint NOT NULL,
    created bigint NOT NULL,
    PRIMARY KEY (game_id, user_id),
    FOREIGN KEY (game_id) REFERENCES game (id),
    FOREIGN KEY (user_id) REFERENCES "user" (id)
);

CREATE TABLE
guess (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,
    game_id uuid NOT NULL,
    updated bigint NOT NULL,
    user_id text NOT NULL,
    word text NOT NULL,
    percentile bigint NOT NULL,
    similarity double precision NOT NULL,
    idx bigint NOT NULL,
    latest_guess_user_id text NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (game_id) REFERENCES game (id),
    FOREIGN KEY (latest_guess_user_id) REFERENCES "user" (id)
);

CREATE TABLE
nearby (
    word text NOT NULL,
    neighbor text NOT NULL,
    similarity double precision NOT NULL,
    percentile bigint NOT NULL,
    PRIMARY KEY (word, neighbor),
    FOREIGN KEY (word) REFERENCES word2vec (word),
    FOREIGN KEY (neighbor) REFERENCES word2vec (word)
);

CREATE TABLE
similarity_range (
    word text NOT NULL,
    top double precision NOT NULL,
    top10 double precision NOT NULL,
    rest double precision NOT NULL,
    PRIMARY KEY (word)
);

