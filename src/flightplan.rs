use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Deserialize, Serialize)]
pub struct User(pub String);

#[derive(Deserialize, Serialize)]
pub struct FlightPlan {}

impl FlightPlan {
    pub async fn fetch(user_id: &str) -> Result<FlightPlan> {
        let url = format!("https://www.simbrief.com/api/xml.fetcher.php?userid={user_id}");
        let body = utils::fetch_url_data(&url).await?;

        serde_json::from_str(&body).context("failed deserializing flightplan")
    }
}
