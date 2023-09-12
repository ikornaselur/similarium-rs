from collections import namedtuple
import os
from functools import partial
from itertools import islice
from pathlib import Path
from typing import Iterable, Iterator

import gensim.models.keyedvectors as word2vec
import psycopg2
from pgvector.psycopg2 import register_vector
from rich.console import Console
from rich.progress import MofNCompleteColumn, Progress, TimeElapsedColumn

SCRIPTS_DIR = Path(__file__).parent

ENGLISH_WORDS = SCRIPTS_DIR / "wordlists" / "english.txt"
BAD_WORDS = SCRIPTS_DIR / "wordlists" / "bad.txt"
VECTORS_PATH = SCRIPTS_DIR / "GoogleNews-vectors-negative300.bin"
DATABASE_URL = os.environ["DATABASE_URL"]

Word = namedtuple("Word", ["name", "vec", "norm"])
Similarities = list[tuple[float, str]]

CHUNK_SIZE = 100

console = Console()


def chunked(iterable: Iterable, n: int = 10_000) -> Iterator:
    def take(n: int, iterable: Iterable) -> list:
        return list(islice(iterable, n))

    return iter(partial(take, n, iter(iterable)), [])


def get_vectors() -> word2vec.KeyedVectors:
    console.log("Load vectors into model")
    with console.status("Importing..."):
        vectors: word2vec.KeyedVectors = word2vec.KeyedVectors.load_word2vec_format(
            VECTORS_PATH, binary=True
        )

    return vectors


def insert(vectors: word2vec.KeyedVectors) -> None:
    conn = psycopg2.connect(DATABASE_URL)
    register_vector(conn)
    cur = conn.cursor()

    console.log("Loading english wordlist")
    with open(ENGLISH_WORDS, "r") as english_words_file:
        english_words = {line.strip() for line in english_words_file.readlines()}

    console.log("Loading bad word list")
    with open(BAD_WORDS, "r") as bad_words_file:
        bad_words = {line.strip() for line in bad_words_file.readlines()}

    wordlist = english_words - bad_words

    with Progress(
        *Progress.get_default_columns(),
        TimeElapsedColumn(),
        MofNCompleteColumn(),
    ) as progress:
        words: list[str]
        for words in chunked(
            progress.track(
                vectors.key_to_index,
                description="Importing model to database...",
            )
        ):
            rows = []
            for word in words:
                if word not in wordlist:
                    continue

                rows.append((word, vectors[word]))

            # Insert whole chunk at once
            cur.executemany(
                "INSERT INTO word2vec (word, vec) VALUES (%s, %s)",
                rows,
            )
            conn.commit()

    conn.commit()
    cur.close()


def main():
    vectors = get_vectors()
    insert(vectors)


if __name__ == "__main__":
    main()
