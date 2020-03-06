use crate::vroom_actor;
use crate::vroom_actor::dto::{VroomJob, VroomRequest, VroomVehicle};
use crate::vrp::dto::{JobsAvailabletimeslot, VrpRequest};
use actix::ArbiterService;
use actix_web::client::Client;
use actix_web::web::{BytesMut, Json};
use actix_web::{web, Error, HttpResponse};
use futures;
use futures::StreamExt;
use std::ops::Deref;

mod dto;
mod postcodes;

pub async fn calculate(data: Json<VrpRequest>) -> Result<HttpResponse, Error> {
    let addr = vroom_actor::VroomActor::from_registry();
    let vroom_request = convert_to_json_vroom(data.0).await;
    let response = addr.send(vroom_request);

    HttpResponse::Ok()
        .content_type("application/json")
        .body(response.await.unwrap())
        .await
}

async fn convert_to_json_vroom(data: VrpRequest) -> VroomRequest {
    let mut vroom_request = VroomRequest {
        vehicles: vec![],
        jobs: vec![],
    };

    let mut vehicle_counter: i64 = 0;
    for vehicle in data.vehicles {
        vehicle_counter += 1;
        let start = convert_postcode_to_coordinates(&vehicle.start_postcode)
            .await
            .unwrap();
        let end = convert_postcode_to_coordinates(&vehicle.end_postcode)
            .await
            .unwrap();

        let vroom_vehicle = VroomVehicle {
            id: vehicle_counter,
            start,
            end,
            capacity: vec![1],
            skills: vec![1],
            profile: "car".to_string(),
            time_window: build_window(&vehicle.time_window).await,
        };
        vroom_request.vehicles.push(vroom_vehicle)
    }

    let mut job_counter: i64 = 0;
    for job in data.jobs {
        job_counter += 1;
        let location = convert_postcode_to_coordinates(&job.postcode)
            .await
            .unwrap();
        let vroom_job = VroomJob {
            id: job_counter,
            service: 0,
            delivery: vec![0],
            location,
            skills: vec![1],
            profile: "car".to_string(),
            pickup: None,
            priority: None,
            time_windows: Option::from(build_windows(&job.available_timeslots).await),
        };
        vroom_request.jobs.push(vroom_job)
    }

    vroom_request
}

async fn build_windows(slots: &Vec<JobsAvailabletimeslot>) -> Vec<Vec<i32>> {
    let mut list = vec![];
    slots
        .iter()
        .for_each(|slot| list.push(vec![slot.start_time, slot.end_time]));
    list
}

async fn build_window(slot: &Vec<i64>) -> Vec<i64> {
    let x = *slot.get(0).unwrap();
    let y = *slot.get(1).unwrap();
    vec![x, y]
}

async fn convert_postcode_to_coordinates(postcode: &String) -> Result<Vec<f64>, Error> {
    let client = Client::default();

    let mut res = client
        .get(format!("http://api.postcodes.io/postcodes/{}", postcode))
        .send()
        .await
        .map_err(Error::from)?;

    let mut body = BytesMut::new();
    while let Some(chunk) = res.next().await {
        body.extend_from_slice(&chunk?);
    }

    let body: postcodes::Json = serde_json::from_slice(&body).unwrap();

    Ok(vec![body.result.longitude, body.result.latitude])
}

pub fn vrp_config(config: &mut web::ServiceConfig) {
    config.service(web::resource("/calculate").route(web::post().to(calculate)));
}
