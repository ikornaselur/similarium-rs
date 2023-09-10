use fang::{
    async_trait,
    asynk::async_queue::AsyncQueueable,
    serde::{Deserialize, Serialize},
    typetag, AsyncRunnable, FangError, Scheduled,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "fang::serde")]
pub struct GameTask;

#[typetag::serde]
#[async_trait]
impl AsyncRunnable for GameTask {
    async fn run(&self, _queueable: &mut dyn AsyncQueueable) -> Result<(), FangError> {
        log::debug!("Running GameTask");
        Ok(())
    }

    fn uniq(&self) -> bool {
        true
    }

    fn cron(&self) -> Option<Scheduled> {
        let expression = "0 0 * * * *";
        Some(Scheduled::CronPattern(expression.to_string()))
    }

    fn backoff(&self, attempt: u32) -> u32 {
        u32::pow(2, attempt)
    }
}
