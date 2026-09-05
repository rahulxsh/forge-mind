use crate::constants::MAX_PROCESS_JOB_ATTEMPTS;
use crate::jobs::channel::Job;
use crate::jobs::job::process_job;
use domain::documents::DocumentStatus;
use crate::repositories::documents::DocumentsRepository;
use extractor::DataLabExtractor;
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
    extractor: Arc<DataLabExtractor>,
    tx: mpsc::Sender<Job>,
) -> JoinSet<()> {
    let mut set = JoinSet::new();
    for worker_id in 0..pool_count {
        let receiver = Arc::clone(&rx);
        let toke_c = token.clone();
        let repo = repository.clone();
        let ext = Arc::clone(&extractor);
        let sender = tx.clone();

        set.spawn(async move {
            loop {
                select! {
                    job = async {
                    let mut rx = receiver.lock().await;
                    rx.recv().await
                    } => {
                        match job {
                            Some(mut j) => {
                                info!(
                                    worker_id,
                                    job_id = j.id.to_string(),
                                    "worker received job"
                                );

                                if let Err(e) = repo.update_status(
                                    j.id,
                                    DocumentStatus::Processing,
                                    j.attempts,
                                    None
                                ).await {
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

                                j.attempts += 1;
                                match process_job(PathBuf::from(document.path),&ext).await {
                                    Ok(_) => {
                                        repo.update_status(
                                            j.id,
                                            DocumentStatus::Processed,
                                            j.attempts,
                                            None
                                        ).await.ok();
                                    },
                                    Err(e) => {

                                        info!("Worker: Failed to process document: {}", e);

                                        if j.attempts < MAX_PROCESS_JOB_ATTEMPTS {
                                            if let Err(e) = repo.update_status(
                                                j.id,
                                                DocumentStatus::Queued,
                                                j.attempts,
                                                Some("Internal: Failed to update document status".into())
                                            ).await {
                                                 info!("Failed to update retry status: {}", e);
                                                 continue;
                                            }

                                            // Put back job to queue
                                            if let Err(e) = sender.send(j).await {
                                                info!("Failed to requeue job:{}",e);
                                            }
                                        } else {
                                            if let Err(e) = repo.update_status(
                                                j.id,
                                                DocumentStatus::Failed,
                                                j.attempts,
                                                Some("Internal: Failed to update document status".into())
                                            ).await {
                                                info!("Failed to update document status:{}",e);
                                            }
                                        }
                                    }
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
