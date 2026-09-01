use crate::jobs::channel::Job;
use crate::jobs::job::process_job;
use std::sync::Arc;
use tokio::select;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::info;

pub async fn create_worker_pool(
    pool_count: usize,
    rx: Arc<Mutex<mpsc::Receiver<Job>>>,
    token: CancellationToken,
) -> JoinSet<()> {
    let mut set = JoinSet::new();
    for worker_id in 0..pool_count {
        let receiver = Arc::clone(&rx);
        let toke_c = token.clone();

        set.spawn(async move {
            loop {
                select! {
                    job = async {
                    let mut rx = receiver.lock().await;
                    rx.recv().await
                    } => {
                        match job {
                            Some(j) => {
                                tracing::info!(
                                    worker_id,
                                    job_id = j.id.to_string(),
                                    "worker received job"
                                );

                                process_job(j).await;
                             },
                             None => {
                                info!("Worker {} shutting down", worker_id);
                                break;
                             }
                        }
                    }

                    _ = toke_c.cancelled() => {
                        info!("WORKER:{} Shutting Down",worker_id);
                        break;
                    }
                }
            }
        });
    }

    set
}
