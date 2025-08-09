use anyhow::{Context, Result};

pub async fn fetch_url_data(url: &str) -> Result<String> {
    reqwest::get(url)
        .await
        .context("unable to fetch url")?
        .text()
        .await
        .context("failed to get text from response")
}
