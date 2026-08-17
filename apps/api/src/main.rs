use anyhow::Result;
use axum::Router;
use axum::routing::get;
use config::{Config, load_config};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    let config: Config = load_config()?;
    info!("Config Loaded");

    let app = Router::new().route("/health", get(health));

    let address = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&address).await?;

    info!(%address,"Service is up and running");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> &'static str {
    "Health"
}
