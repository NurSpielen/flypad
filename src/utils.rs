use anyhow::{Context, Result};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub async fn fetch_url_data(url: &str) -> Result<String> {
    reqwest::get(url)
        .await
        .context("unable to fetch url")?
        .text()
        .await
        .context("failed to get text from response")
}

// Simbrief has decided to return empty objects instead of strings for some reason
// This function allows me to avoid having the untagged StringOrEmptyObject enum in my structs
pub fn deserialize_flight_plan_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;

    match value {
        Value::String(s) => Ok(s),
        _ => Ok("No Value".to_string()),
    }
}

// AviationWeather.gov occasionally returns the visibility as a number instead of as a string.
// This handles that
pub fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;

    match value {
        Value::String(s) => Ok(Some(s)),
        Value::Number(n) => Ok(Some(n.to_string())),
        _ => Ok(None),
    }
}
