use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Weather {
    #[serde(rename = "temp")]
    pub temperature: f32,
    #[serde(rename = "dewp")]
    pub dew_point: f32,
    #[serde(rename = "wdir")]
    pub wind_direction: f32,
    #[serde(rename = "wspd")]
    pub wind_speed: f32,
    #[serde(rename = "wgst")]
    pub wind_gust: Option<f32>,
    #[serde(rename = "visib")]
    pub visibility: String,
    #[serde(rename = "altim")]
    pub altimeter: f32,
    #[serde(rename = "rawOb")]
    pub metar: String,
    #[serde(rename = "rawTaf")]
    pub taf: Option<String>,
}

impl Weather {
    pub async fn fetch(icao: &str, should_fetch_taf: bool) -> Result<Weather> {
        let url = format!(
            "https://aviationweather.gov/api/data/metar?ids={icao}&format=json&taf={should_fetch_taf}"
        );

        let body = reqwest::get(url)
            .await
            .context("unable to fetch url")?
            .text()
            .await
            .context("failed to get text from response")?;

        // This is being done as the weather is provided as an array
        let mut weather: Vec<Weather> =
            serde_json::from_str(&body).context("failed to deserialize weather")?;

        Ok(weather.remove(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_fetch() -> anyhow::Result<()> {
        let weather = Weather::fetch("KJFK", true).await?;
        println!("{weather:?}");
        Ok(())
    }
}
