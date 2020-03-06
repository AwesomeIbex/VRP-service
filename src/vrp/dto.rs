use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct JobsAvailabletimeslot {
    pub start_time: i32,
    pub end_time: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub postcode: String,
    pub skills: Vec<String>,
    pub available_timeslots: Vec<JobsAvailabletimeslot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vehicle {
    pub start_postcode: String,
    pub end_postcode: String,
    pub capacity: i64,
    pub skills: Vec<String>,
    pub time_window: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VrpRequest {
    pub jobs: Vec<Job>,
    pub vehicles: Vec<Vehicle>,
    pub matrix: Vec<Vec<i64>>,
}
