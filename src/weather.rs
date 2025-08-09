use anyhow::{Context, Result};
use serde::{de::Error, Deserialize};

use crate::utils;


// This provides getters to the fields instead of making these public as I have found out that
// the responses may contain null values, therefore I have decided to avoid checking if the value
// exists within the application, and instead I just return the default value if needed.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Weather {
    #[serde(rename = "temp")]
    temperature: Option<f32>,
    #[serde(rename = "dewp")]
    dew_point: Option<f32>,
    #[serde(rename = "wdir")]
    wind_direction: Option<f32>,
    #[serde(rename = "wspd")]
    wind_speed: Option<f32>,
    #[serde(rename = "wgst")]
    wind_gust: Option<f32>,
    #[serde(deserialize_with = "utils::deserialize_optional_string")]
    #[serde(rename = "visib")]
    visibility: Option<String>,
    #[serde(rename = "altim")]
    altimeter: Option<f32>,
    #[serde(rename = "rawOb")]
    pub metar: String,
    #[serde(rename = "rawTaf")]
    pub taf: Option<String>,
}

impl Weather {
    pub fn temperature(&self) -> f32 {
        self.temperature.unwrap_or_default()
    }

    pub fn dew_point(&self) -> f32 {
        self.dew_point.unwrap_or_default()
    }

    pub fn wind_direction(&self)-> f32{
        self.wind_direction.unwrap_or_default()
    }

    pub fn wind_speed(&self) -> f32 {
        self.wind_speed.unwrap_or_default()
    }

    pub fn wind_gust(&self) -> f32 {
        self.wind_gust.unwrap_or_default()
    }

    pub fn visibility(&self) -> &str {
        match &self.visibility {
            Some(val) => val,
            None => ""
        }
    }

    pub fn altimeter(&self) -> f32 {
        self.altimeter.unwrap_or_default()
    }

    pub async fn fetch(icao: &str, should_fetch_taf: bool) -> Result<Weather> {
        let url = format!(
            "https://aviationweather.gov/api/data/metar?ids={icao}&format=json&taf={should_fetch_taf}"
        );

        let body = utils::fetch_url_data(&url).await?;

        // This is being done as the weather is provided as an array
        let mut weather: Vec<Weather> =
            serde_json::from_str(&body).context("failed to deserialize weather")?;

        if weather.is_empty() {
            Err(serde_json::Error::custom("empty weather response"))
                .context("invalid weather response received")?;
        }

        Ok(weather.remove(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_fetch() -> anyhow::Result<()> {
        let weather = Weather::fetch("RKSI", true).await?;
        println!("{weather:?}");
        Ok(())
    }
}
