use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word2Vec {
    pub word: String,
    pub vec: Vec<u8>,
}

impl Word2Vec {
    pub async fn get(pool: &SqlitePool, word: String) -> anyhow::Result<Option<Word2Vec>> {
        let result = sqlx::query!(
            r#"
            SELECT *
            FROM word2vec
            WHERE word = ?
            "#,
            word
        )
        .fetch_optional(pool)
        .await?;

        if let Some(word2vec) = result {
            Ok(Some(Word2Vec {
                word: word2vec.word,
                vec: word2vec.vec,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn expanded_vec(&self) -> Vec<f32> {
        let mut exp_vec: Vec<f32> = Vec::new();
        let mut vec_iter = self.vec.iter();

        while let Some(a) = vec_iter.next() {
            if let Some(b) = vec_iter.next() {
                exp_vec.push(f32::from_le_bytes([0, 0, *a, *b]));
            }
        }

        exp_vec
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expanded_vec() {
        let w2v = Word2Vec {
            word: "foo".to_string(),
            vec: vec![101, 62, 222, 62],
        };

        let expected = vec![0.22363281, 0.43359375];

        assert_eq!(w2v.expanded_vec(), expected);
    }
}
