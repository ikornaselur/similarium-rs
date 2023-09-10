use fang::{
    async_trait,
    asynk::async_queue::AsyncQueueable,
    serde::{Deserialize, Serialize},
    typetag, AsyncRunnable, FangError, Scheduled,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "fang::serde")]
pub struct DebugTask {
    pub msg: String,
}

#[typetag::serde]
#[async_trait]
impl AsyncRunnable for DebugTask {
    async fn run(&self, _queueable: &mut dyn AsyncQueueable) -> Result<(), FangError> {
        log::debug!("Debug task: {}", self.msg);
        Ok(())
    }

    // If `uniq` is set to true and the task is already in the storage, it won't be inserted again
    // The existing record will be returned for for any insertions operaiton
    fn uniq(&self) -> bool {
        true
    }

    // This will be useful if you would like to schedule tasks.
    // default value is None (the task is not scheduled, it's just executed as soon as it's inserted)
    fn cron(&self) -> Option<Scheduled> {
        None
        // let expression = "* * * * * *";
        // Some(Scheduled::CronPattern(expression.to_string()))
    }

    // backoff mode for retries
    fn backoff(&self, attempt: u32) -> u32 {
        u32::pow(2, attempt)
    }
}
