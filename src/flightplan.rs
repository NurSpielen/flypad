use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User(pub String);

pub struct FlightPlan {}
