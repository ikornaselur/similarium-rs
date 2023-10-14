use crate::{db::get_pool, models::Word2Vec};
use fang::{
    async_trait,
    asynk::async_queue::AsyncQueueable,
    serde::{Deserialize, Serialize},
    typetag, AsyncRunnable, FangError, Scheduled,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "fang::serde")]
pub struct MatViewCleanupTask {}

#[typetag::serde]
#[async_trait]
impl AsyncRunnable for MatViewCleanupTask {
    async fn run(&self, _queue: &mut dyn AsyncQueueable) -> Result<(), FangError> {
        log::debug!("Running MatViewCleanup");
        let pool = get_pool();

        Word2Vec::cleanup_materialised_views(pool).await?;

        Ok(())
    }

    fn uniq(&self) -> bool {
        true
    }

    fn cron(&self) -> Option<Scheduled> {
        let expression = "0 17 4 * * *".to_string();
        Some(Scheduled::CronPattern(expression))
    }

    fn backoff(&self, attempt: u32) -> u32 {
        u32::pow(2, attempt)
    }
}
