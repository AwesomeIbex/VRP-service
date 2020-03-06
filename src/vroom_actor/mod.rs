use crate::vroom_actor::dto::VroomRequest;
use actix::{Actor, ArbiterService, Context, Handler, Message};
use std::process::Command;

pub(crate) mod dto;

pub fn calculate(data: VroomRequest) -> String {
    let value = serde_json::to_string(&data).unwrap();
    let arg = format!("/var/tmp/vroom -g '{}'", value.as_str());
    let output = Command::new("sh")
        .arg("-c")
        .arg(arg)
        .output()
        .expect("failed to execute process");

    let route = std::str::from_utf8(output.stdout.as_slice());

    route.unwrap().to_string()
}

impl Message for VroomRequest {
    type Result = String;
}

impl Handler<VroomRequest> for VroomActor {
    type Result = String;

    fn handle(&mut self, msg: VroomRequest, _ctx: &mut Self::Context) -> Self::Result {
        calculate(msg)
    }
}

#[derive(Default)]
pub struct VroomActor {}

impl actix::Supervised for VroomActor {}

impl ArbiterService for VroomActor {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        println!("VROOM Actor started");
    }
}

impl Actor for VroomActor {
    type Context = Context<Self>;
}
