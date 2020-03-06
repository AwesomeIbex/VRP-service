use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonSummaryComputingTime {
    pub loading: i64,
    pub solving: i64,
    pub routing: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonSummary {
    pub cost: i64,
    pub unassigned: i64,
    pub delivery: Vec<i64>,
    pub amount: Vec<i64>,
    pub pickup: Vec<i64>,
    pub service: i64,
    pub duration: i64,
    pub waiting_time: i64,
    pub distance: i64,
    pub computing_times: JsonSummaryComputingTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRoutesStep {
    #[serde(rename = "type")]
    pub type2: String,
    pub location: Vec<f64>,
    pub load: Vec<i64>,
    pub arrival: i64,
    pub duration: i64,
    pub distance: i64,
    pub job: i64,
    pub service: i64,
    pub waiting_time: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRoute {
    pub vehicle: i64,
    pub cost: i64,
    pub delivery: Vec<i64>,
    pub amount: Vec<i64>,
    pub pickup: Vec<i64>,
    pub service: i64,
    pub duration: i64,
    pub waiting_time: i64,
    pub distance: i64,
    pub steps: Vec<JsonRoutesStep>,
    pub geometry: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Json {
    pub code: i64,
    pub summary: JsonSummary,
    pub unassigned: Vec<i64>,
    pub routes: Vec<JsonRoute>,
}
