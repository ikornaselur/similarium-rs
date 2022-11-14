use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nearby {
    pub word: String,
    pub neighbor: String,
    pub similarity: f64,
    pub percentile: i64,
}

impl Nearby {
    pub async fn get(
        pool: &SqlitePool,
        word: String,
        neighbor: String,
    ) -> anyhow::Result<Option<Nearby>> {
        let result = sqlx::query!(
            r#"
            SELECT *
            FROM nearby
            WHERE word = ?1 AND neighbor = ?2
            "#,
            word,
            neighbor
        )
        .fetch_optional(pool)
        .await?;

        if let Some(nearby) = result {
            Ok(Some(Nearby {
                word: nearby.word,
                neighbor: nearby.neighbor,
                similarity: nearby.similarity,
                percentile: nearby.percentile,
            }))
        } else {
            Ok(None)
        }
    }
}
