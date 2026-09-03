use crate::AppState;
use crate::jobs::channel::Job;
use axum::extract::State;
use std::path::PathBuf;
use tokio::time::sleep;
use uuid::Uuid;

pub async fn start_job(State(state): State<AppState>) -> Result<String, String> {
    let id = Uuid::new_v4();
    state.tx.send(Job { id }).await.map_err(|e| e.to_string())?;

    Ok("Accepted".into())
}

pub async fn process_job(path: PathBuf) -> Result<(), std::io::Error> {
    let data = tokio::fs::read_to_string(path).await?;
    println!("Data:{}", data);
    sleep(std::time::Duration::from_secs(10)).await;
    Ok(())
}
