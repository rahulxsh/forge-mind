use crate::jobs::channel::Job;
use std::sync::Arc;
use tokio::select;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::task::JoinSet;
use tokio::time::sleep;
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
                            info!("Worker ID:{}, JOB:{:?}",worker_id,j);
                            sleep(std::time::Duration::from_secs(15)).await;
                            info!("DONE: Worker ID:{}, JOB:{:?}",worker_id,j);
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
                };
            }
        });
    }

    set
}
