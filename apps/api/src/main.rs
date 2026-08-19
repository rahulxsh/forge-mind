mod jobs;
mod workers;

use crate::jobs::channel::Job;
use crate::jobs::job::start_job;
use crate::workers::worker::create_worker_pool;
use anyhow::Result;
use axum::Router;
use axum::routing::{get, post};
use config::{Config, load_config};
use jobs::channel::JobChannel;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{Mutex, mpsc};
use tokio_util::sync::CancellationToken;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub tx: mpsc::Sender<Job>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let token = CancellationToken::new();
    let token_cloned = token.clone();
    let job_channel = JobChannel::new(2);
    let pool_count: usize = 10;
    tracing_subscriber::fmt().init();
    let config: Config = load_config()?;
    info!("Config Loaded");

    let state = AppState { tx: job_channel.tx };
    let app: Router = Router::new()
        .route("/health", get(health))
        .route("/job", post(start_job))
        .with_state(state);

    let worker_pool_handle = tokio::spawn(async move {
        let mut set = create_worker_pool(
            pool_count,
            Arc::new(Mutex::new(job_channel.rx)),
            token_cloned,
        )
        .await;

        while let Some(value) = set.join_next().await {
            match value {
                Ok(_) => {}
                Err(_e) => {
                    info!("WORKER: Task Failed")
                }
            }
        }
    });

    let address = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&address).await?;

    info!(%address,"Service is up and running");
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen for Ctrl+C");
            info!("\nShutdown signal received");
            token.cancel();
        })
        .await?;

    worker_pool_handle.await?;

    Ok(())
}

async fn health() -> &'static str {
    "Health"
}
