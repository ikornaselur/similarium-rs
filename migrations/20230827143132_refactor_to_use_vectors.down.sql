DROP TABLE word2vec;
CREATE TABLE
word2vec (
    word text NOT NULL,
    vec bytea NOT NULL,
    PRIMARY KEY (word)
);

DROP EXTENSION vectors;

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
