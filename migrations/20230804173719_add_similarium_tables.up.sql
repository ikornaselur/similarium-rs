CREATE TABLE
public.channel (
    id text NOT NULL,
    team_id text NOT NULL,
    hour bigint NOT NULL,
    active boolean NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE
public."user" (
    id text NOT NULL,
    profile_photo text NOT NULL,
    username text NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE
public.word2vec (
    word text NOT NULL,
    vec bytea NOT NULL,
    PRIMARY KEY (word)
);

CREATE TABLE
public.game (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,
    channel_id text NOT NULL,
    thread_ts text NOT NULL,
    puzzle_number bigint NOT NULL,
    date text NOT NULL,
    active boolean NOT NULL,
    secret text NOT NULL,
    hint text,
    PRIMARY KEY (id),
    FOREIGN KEY (channel_id) REFERENCES public.channel (id)
);

CREATE INDEX channel_thread_idx ON public.game USING btree (
    channel_id, thread_ts
);

CREATE TABLE
public.game_user_hint_association (
    game_id uuid NOT NULL,
    user_id text NOT NULL,
    created bigint NOT NULL,
    guess_idx integer NOT NULL,
    PRIMARY KEY (game_id, user_id),
    FOREIGN KEY (game_id) REFERENCES public.game (id),
    FOREIGN KEY (user_id) REFERENCES public."user" (id)
);

CREATE TABLE
public.game_user_winner_association (
    game_id uuid NOT NULL,
    user_id text NOT NULL,
    guess_idx bigint NOT NULL,
    created bigint NOT NULL,
    PRIMARY KEY (game_id, user_id),
    FOREIGN KEY (game_id) REFERENCES public.game (id),
    FOREIGN KEY (user_id) REFERENCES public."user" (id)
);

CREATE TABLE
public.guess (
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
    FOREIGN KEY (game_id) REFERENCES public.game (id),
    FOREIGN KEY (latest_guess_user_id) REFERENCES public."user" (id)
);

CREATE TABLE
public.nearby (
    word text NOT NULL,
    neighbor text NOT NULL,
    similarity double precision NOT NULL,
    percentile bigint NOT NULL,
    PRIMARY KEY (word, neighbor),
    FOREIGN KEY (word) REFERENCES public.word2vec (word),
    FOREIGN KEY (neighbor) REFERENCES public.word2vec (word)
);

CREATE TABLE
public.similarity_range (
    word text NOT NULL,
    top double precision NOT NULL,
    top10 double precision NOT NULL,
    rest double precision NOT NULL,
    PRIMARY KEY (word)
);

