use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    id: String,
    team_id: String,
    hour: i64,
    active: bool,
}

impl Channel {
    pub async fn get(pool: &SqlitePool, id: String) -> anyhow::Result<Option<Channel>> {
        let result = sqlx::query!(
            r#"
            SELECT *
            FROM channel
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(channel) = result {
            Ok(Some(Channel {
                id: channel.id,
                team_id: channel.team_id,
                hour: channel.hour,
                active: channel.active,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_by_hour(pool: &SqlitePool, hour: i64) -> anyhow::Result<Vec<Channel>> {
        let results = sqlx::query!(
            r#"
            SELECT *
            FROM channel
            WHERE hour = ?
            "#,
            hour
        )
        .fetch_all(pool)
        .await?;

        let channels = results
            .iter()
            .map(|row| Channel {
                id: row.id.clone(),
                team_id: row.team_id.clone(),
                hour,
                active: row.active,
            })
            .collect();

        Ok(channels)
    }

    pub async fn new(
        pool: &SqlitePool,
        id: String,
        team_id: String,
        hour: i64,
        active: bool,
    ) -> anyhow::Result<Channel> {
        sqlx::query!(
            r#"
            INSERT INTO channel (id, team_id, hour, active)
            VALUES (?1, ?2, ?3, ?4)
            "#,
            id,
            team_id,
            hour,
            active,
        )
        .execute(pool)
        .await?;

        Ok(Channel {
            id,
            team_id,
            hour,
            active,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use sqlx::SqlitePool;

    #[sqlx::test]
    async fn test_creating_channel(pool: SqlitePool) -> anyhow::Result<()> {
        let channel_id = String::from("channel_x");
        let team_id = String::from("team_x");
        let hour = 8;
        let active = true;

        let channel =
            Channel::new(&pool, channel_id.clone(), team_id.clone(), hour, active).await?;

        assert_eq!(channel.id, channel_id);
        assert_eq!(channel.team_id, team_id);
        assert_eq!(channel.hour, hour);
        assert_eq!(channel.active, active);

        Ok(())
    }

    #[sqlx::test]
    async fn test_getting_channel_by_id(pool: SqlitePool) -> anyhow::Result<()> {
        let team_id = String::from("team_x");
        Channel::new(&pool, String::from("channel_1"), team_id.clone(), 8, true).await?;
        Channel::new(&pool, String::from("channel_2"), team_id.clone(), 8, true).await?;

        let channel = Channel::get(&pool, String::from("channel_2")).await?;

        assert!(channel.is_some());

        let channel = channel.unwrap();
        assert_eq!(channel.id, String::from("channel_2"));

        Ok(())
    }

    #[sqlx::test]
    async fn test_getting_channels_by_hour(pool: SqlitePool) -> anyhow::Result<()> {
        let team_id = String::from("team_x");
        let hour = 8;
        Channel::new(
            &pool,
            String::from("channel_1"),
            team_id.clone(),
            hour,
            true,
        )
        .await?;
        Channel::new(
            &pool,
            String::from("channel_2"),
            team_id.clone(),
            hour,
            true,
        )
        .await?;

        let channels = Channel::get_by_hour(&pool, hour).await?;
        assert_eq!(channels.len(), 2);
        let mut channel_ids = channels
            .iter()
            .map(|channel| channel.id.clone())
            .collect::<Vec<String>>();
        channel_ids.sort();

        assert_eq!(
            channel_ids,
            vec![String::from("channel_1"), String::from("channel_2")]
        );

        let no_channels = Channel::get_by_hour(&pool, hour + 1).await?;
        assert_eq!(no_channels.len(), 0);

        Ok(())
    }
}
