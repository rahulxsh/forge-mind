use crate::jobs::channel::Job;
use crate::jobs::job::process_job;
use crate::models::documents::DocumentStatus;
use crate::repositories::documents::DocumentsRepository;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::select;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::info;

pub async fn create_worker_pool(
    repository: DocumentsRepository,
    pool_count: usize,
    rx: Arc<Mutex<mpsc::Receiver<Job>>>,
    token: CancellationToken,
) -> JoinSet<()> {
    let mut set = JoinSet::new();
    for worker_id in 0..pool_count {
        let receiver = Arc::clone(&rx);
        let toke_c = token.clone();
        let repo = repository.clone();

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

                                if let Err(e) = repo.update_status(j.id,DocumentStatus::Processing).await {
                                    info!("Failed to update document status: {}", e);
                                    continue;
                                }

                                let document = match repo.get_by_id(j.id).await {
                                    Ok(Some(document)) => document,
                                    Ok(None) => {
                                        info!("Document Not Found");
                                        continue;
                                    }
                                    Err(e) => {
                                        info!("Failed to get document:{}",e);
                                        continue;
                                    }
                                };

                                let status = match process_job(PathBuf::from(document.path)).await {
                                    Ok(_) => DocumentStatus::Processed,
                                    Err(e) => {
                                        info!("Worker: Failed to process document: {}", e);
                                        DocumentStatus::Failed
                                    }
                                };

                                if let Err(e) = repo.update_status(j.id, status).await {
                                    info!("Failed to update document status: {}", e);
                                    continue;
                                }
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
