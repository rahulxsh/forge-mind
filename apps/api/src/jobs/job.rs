use crate::AppState;
use crate::jobs::channel::Job;
use axum::extract::State;
use tokio::time::sleep;
use uuid::Uuid;

pub async fn start_job(State(state): State<AppState>) -> Result<String, String> {
    let id = Uuid::new_v4();
    state
        .tx
        .send(Job {
            id,
        })
        .await
        .map_err(|e| e.to_string())?;

    Ok("Accepted".into())
}


#[tracing::instrument(
    name = "process_job",
    skip(job),
    fields(job_id = job.id.to_string())
)]
pub async fn process_job(job: Job) {
    tracing::info!("processing job");

   sleep(std::time::Duration::from_secs(10)).await;

    tracing::info!("job completed");
}