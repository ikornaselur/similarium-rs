use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityRange {
    pub word: String,
    pub top: f64,
    pub top10: f64,
    pub rest: f64,
}

impl SimilarityRange {
    pub async fn get(pool: &SqlitePool, word: String) -> anyhow::Result<Option<SimilarityRange>> {
        let result = sqlx::query!(
            r#"
            SELECT *
            FROM similarity_range
            WHERE word = ?
            "#,
            word
        )
        .fetch_optional(pool)
        .await?;

        if let Some(similarity_range) = result {
            Ok(Some(SimilarityRange {
                word: similarity_range.word,
                top: similarity_range.top,
                top10: similarity_range.top10,
                rest: similarity_range.rest,
            }))
        } else {
            Ok(None)
        }
    }
}
