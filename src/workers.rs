use crate::SimilariumError;
use fang::{
    asynk::{async_queue::AsyncQueue, async_worker_pool::AsyncWorkerPool},
    NoTls,
};

pub async fn start_workers(
    database_url: &str,
    worker_count: u32,
    max_pool_size: u32,
) -> Result<(), SimilariumError> {
    log::info!("Starting worker pool with {} workers", worker_count);

    let mut queue = AsyncQueue::builder()
        .uri(database_url)
        .max_pool_size(max_pool_size)
        .build();
    queue.connect(NoTls).await.unwrap();

    let mut pool: AsyncWorkerPool<AsyncQueue<NoTls>> = AsyncWorkerPool::builder()
        .number_of_workers(worker_count)
        .queue(queue.clone())
        .build();

    pool.start().await;

    Ok(())
}
