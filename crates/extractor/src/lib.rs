use reqwest::Client;
use reqwest::multipart::{Form, Part};
use std::path::PathBuf;
use std::time::Duration;
use domain::datalab::{ResponseData,ResponsePoolData};

pub struct DataLabExtractor {
    client: Client,
    api_key: String,
}

impl DataLabExtractor {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
    pub async fn extract(
        &self,
        path: PathBuf,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let file_part = Part::file(path).await?;
        let form = Form::new()
            .part("file", file_part)
            .text("output_format", "markdown")
            .text("mode", "balanced");

        let response: ResponseData = self
            .client
            .post("https://www.datalab.to/api/v1/convert")
            .header("X-API-Key", &self.api_key)
            .multipart(form)
            .send()
            .await?
            .json()
            .await?;

        let pool_url = format!(
            "https://www.datalab.to/api/v1/convert/{}",
            response.request_id
        );

        loop {
            let pool_response: ResponsePoolData = self
                .client
                .get(&pool_url)
                .header("X-API-Key", &self.api_key)
                .send()
                .await?
                .json()
                .await?;

            println!("Status: {}", pool_response.status);

            match pool_response.status.to_lowercase().as_str() {
                "complete" | "success" => return Ok(pool_response.markdown.unwrap_or_default()),
                "failed" | "error" => return Err("Document conversion failed on the server".into()),
                _ => {
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
            }
        }
    }
}
