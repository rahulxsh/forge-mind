use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool};
use tracing::info;

pub async fn connect_db(url: &String) -> Result<PgPool, Error> {
    let pool = PgPoolOptions::new()
        .min_connections(3)
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(url)
        .await?;

    info!("DB Connected :)");
    Ok(pool)
}
