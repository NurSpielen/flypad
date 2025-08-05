use crate::weather::Weather;

#[derive(Default)]
pub struct Airport {
    pub icao: String,
    pub transition_level: String,
    pub runway: String,
    pub weather: Weather,
}
