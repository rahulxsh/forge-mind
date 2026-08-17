use dotenvy::{Error};
use std::env;

#[derive(Debug)]
pub struct Config {
    pub port: String,
    pub host: String,
}

pub fn load_config() -> Result<Config, Error> {
    let port = env::var("PORT").expect("PORT is missing");

    let host = env::var("HOST").expect("HOST is missing");

    Ok(Config { port, host })
}
