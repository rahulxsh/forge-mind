use sqlx::Error;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub async fn postgresql_client(url: &str) -> Result<PgPool, Error> {
    let pool = PgPoolOptions::new()
        .min_connections(2)
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(url)
        .await?;

    Ok(pool)
}
