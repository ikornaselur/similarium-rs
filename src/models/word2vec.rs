use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Word2Vec {
    pub word: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Similarity {
    pub word: String,
    pub rank: i64,
    pub similarity: f64,
}

impl Word2Vec {
    /// Create the materialised view for this word
    ///
    /// This will calculate the similarity for all words against the target word along with the
    /// rank of the similarity
    /// This is used to optimise the lookup of the rank and similarity for a given word during the
    /// game day.
    pub async fn create_materialised_view(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            format!(
                r#"DROP MATERIALIZED VIEW IF EXISTS word2vec_{0}"#,
                self.word
            )
            .as_str(),
        )
        .execute(db)
        .await?;

        sqlx::query(
            format!(
                r#"
            CREATE MATERIALIZED VIEW word2vec_{0} AS
            SELECT
              a.word,
              s.rank - 1 as rank,
              s.similarity * -100 as similarity
            FROM
              word2vec a
            LEFT JOIN (
              SELECT
                a.word,
                (a.vec <=> b.vec) AS similarity,
                ROW_NUMBER() OVER (ORDER BY (a.vec <=> b.vec)) AS rank
              FROM
                word2vec AS a
              LEFT JOIN
                word2vec AS b on b.word = '{0}'
            ) AS s ON a.word = s.word
            WITH DATA
            "#,
                self.word
            )
            .as_str(),
        )
        .execute(db)
        .await?;

        sqlx::query(
            format!(
                r#"
            CREATE UNIQUE INDEX word2vec_{0}_idx ON word2vec_{0} (word)
            "#,
                self.word
            )
            .as_str(),
        )
        .execute(db)
        .await?;

        Ok(())
    }

    /// Get the rank and similarity of a provided word against the target word
    ///
    /// The word itself will have rank 0 and similarity at 1.0, with the next being below 1.0 and
    /// rank 1 and so on.
    pub async fn get_similarity(
        &self,
        word: &str,
        db: &sqlx::PgPool,
    ) -> Result<Similarity, sqlx::Error> {
        let q = format!(
            r#"
            SELECT
                word, rank, similarity
            FROM
                word2vec_{}
            WHERE
                word = $1
            "#,
            self.word
        );
        let row: (String, i64, f64) = sqlx::query_as(q.as_str()).bind(word).fetch_one(db).await?;

        Ok(Similarity {
            word: row.0,
            rank: row.1,
            similarity: row.2,
        })
    }
}
