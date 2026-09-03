use dotenvy::Error;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub port: String,
    pub host: String,
    pub db_url: String,
    pub test_db_url: String,
    pub datalab_api_key: String,
}

pub fn load_config() -> Result<Config, Error> {
    let port = env::var("PORT").expect("PORT is missing");
    let host = env::var("HOST").expect("HOST is missing");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
    let test_db_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL is missing");
    let datalab_api_key = env::var("DATALAB_API_KEY").expect("DATALAB_API_KEY is missing");

    Ok(Config {
        port,
        host,
        db_url,
        test_db_url,
        datalab_api_key,
    })
}
