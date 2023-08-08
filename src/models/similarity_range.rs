use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct SimilarityRange {
    pub word: String,
    pub top: f64,
    pub top10: f64,
    pub rest: f64,
}

impl SimilarityRange {
    pub async fn get(word: &str, db: &sqlx::PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            SimilarityRange,
            r#"
            SELECT word, top, top10, rest
            FROM similarity_range
            WHERE word = $1
            "#,
            word
        )
        .fetch_one(db)
        .await
    }
}
