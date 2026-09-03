use crate::AppState;
use crate::jobs::channel::Job;
use axum::extract::State;
use extractor::DataLabExtractor;
use std::path::PathBuf;
use uuid::Uuid;

pub async fn start_job(State(state): State<AppState>) -> Result<String, String> {
    let id = Uuid::new_v4();
    state.tx.send(Job { id }).await.map_err(|e| e.to_string())?;

    Ok("Accepted".into())
}

pub async fn process_job(
    path: PathBuf,
    extractor: &DataLabExtractor,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let data = extractor.extract(path).await?;
    println!("Data:{}", data);
    Ok(())
}
