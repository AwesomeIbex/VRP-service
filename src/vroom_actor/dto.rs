use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VroomRequest {
    pub vehicles: Vec<VroomVehicle>,
    pub jobs: Vec<VroomJob>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VroomVehicle {
    pub id: i64,
    pub start: Vec<f64>,
    pub end: Vec<f64>,
    pub capacity: Vec<i64>,
    pub skills: Vec<i64>,
    #[serde(default = "Service::default")]
    pub profile: String,
    pub time_window: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VroomJob {
    pub id: i64,
    pub service: i64,
    pub delivery: Vec<i64>,
    pub location: Vec<f64>,
    pub skills: Vec<i64>,
    #[serde(default = "Service::default")]
    pub profile: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Vec<i16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_windows: Option<Vec<Vec<i32>>>,
}

#[derive(Serialize, Deserialize, Debug, AsStaticStr)]
enum Service {
    Car,
    Pedestrian,
}

impl Service {
    fn default() -> String {
        "Car".to_lowercase().to_string()
    }
}
