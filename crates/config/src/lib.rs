use dotenvy::Error;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub port: String,
    pub host: String,
    pub db_url: String,
}

pub fn load_config() -> Result<Config, Error> {
    let port = env::var("PORT").expect("PORT is missing");

    let host = env::var("HOST").expect("HOST is missing");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is missing");

    Ok(Config { port, host, db_url })
}
