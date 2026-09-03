use anyhow::Result;
use api::db::connect_db;
use api::jobs::channel::JobChannel;
use api::repositories::documents::DocumentsRepository;
use api::services::documents::DocumentService;
use api::workers::worker::create_worker_pool;
use api::{AppState, create_app};
use config::{Config, load_config};
use std::sync::Arc;
use tokio::fs::create_dir_all;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let upload_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("uploads");

    create_dir_all(&upload_dir).await?;

    let config: Config = load_config()?;
    info!("Config Loaded");

    let pool = connect_db(&config.db_url).await?;

    let token = CancellationToken::new();
    let token_cloned = token.clone();

    let job_channel = JobChannel::new(2);
    let pool_count: usize = 10;

    let state = AppState {
        tx: job_channel.tx.clone(),
        document_service: Arc::new(DocumentService {
            repository: DocumentsRepository { pool: pool.clone() },
            tx: job_channel.tx,
        }),
    };

    let app = create_app(state);

    let worker_pool_handle = tokio::spawn(async move {
        let repo = DocumentsRepository { pool: pool.clone() };
        let mut set = create_worker_pool(
            repo,
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
