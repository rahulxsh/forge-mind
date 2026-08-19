use crate::AppState;
use crate::jobs::channel::Job;
use axum::extract::State;

pub async fn start_job(State(state): State<AppState>) -> Result<String, String> {
    state
        .tx
        .send(Job {
            id: "test_job_id".into(),
        })
        .await
        .map_err(|e| e.to_string())?;

    Ok("Accepted".into())
}
