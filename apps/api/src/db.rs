use database::postgresql_client;
use sqlx::{Error, PgPool};
use tracing::info;

pub async fn connect_db(url: &str) -> Result<PgPool, Error> {
    let pool = postgresql_client(url).await?;
    info!("DB Connected :)");
    Ok(pool)
}
