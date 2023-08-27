DROP TABLE similarity_range;
DROP TABLE nearby;

-- Enable the vector extension
CREATE EXTENSION IF NOT EXISTS vectors;

-- Replace the word2vec table
DROP TABLE word2vec;
CREATE TABLE
word2vec (
  word text NOT NULL,
  vec vector(300) NOT NULL,
  PRIMARY KEY (word)
);
