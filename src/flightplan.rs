use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Deserialize, Serialize)]
pub struct User(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Airport {
    #[serde(default)]
    pub icao_code: String,
    #[serde(default)]
    pub iata_code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub plan_rwy: String,
    #[serde(default)]
    pub trans_alt: String,
    #[serde(default)]
    pub trans_level: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FlightOverview {
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub icao_airline: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub flight_number: String,
    #[serde(rename = "costindex")]
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub cost_index: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub route_distance: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub air_distance: String,
    #[serde(rename = "stepclimb_string")]
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub step_climb_string: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub initial_altitude: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub route_ifps: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub route_navigraph: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub sid_ident: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub sid_trans: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub star_ident: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub star_trans: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fuel {
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub taxi: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub enroute_burn: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub contingency: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub alternate_burn: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub reserve: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub etops: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub extra: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub extra_required: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub extra_optional: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub min_takeoff: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub plan_takeoff: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub plan_ramp: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub plan_landing: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub avg_fuel_flow: String,
    #[serde(default)]
    #[serde(deserialize_with = "utils::deserialize_flight_plan_string")]
    pub max_tanks: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FlightPlan {
    pub origin: Airport,
    pub destination: Airport,
    #[serde(rename = "general")]
    pub flight_information: FlightOverview,
    pub fuel: Fuel,
}

impl FlightPlan {
    pub async fn fetch(user_id: &str) -> Result<FlightPlan> {
        // FIXME Remove this to prevent having my user id static.
        let user_id = if user_id.is_empty() {
            "791411"
        } else {
            user_id
        };
        let url = format!("https://www.simbrief.com/api/xml.fetcher.php?userid={user_id}&json=1");
        let body = utils::fetch_url_data(&url).await?;

        serde_json::from_str(&body).context("failed deserializing flightplan")
    }
}
