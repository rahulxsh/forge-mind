use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct ResponseData {
    pub request_id: String,
}

#[derive(Deserialize, Debug)]
pub struct ResponsePoolData {
    pub status: String,
    pub markdown: Option<String>,
}