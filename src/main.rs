
extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::vroom_actor::VroomActor;
use actix::Actor;
use actix_rt::Arbiter;
use actix_web::{web, App, HttpServer};

mod osrm;
mod root;
mod tutorial;
mod vroom_actor;
mod vrp;

fn initialize_actors() {
    Arbiter::new().exec_fn(|| {
        VroomActor::start;
    });
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    set_environment();
    initialize_actors();

    HttpServer::new(|| {
        App::new()
            .data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (global configuration)
            .configure(root::root_config)
            .service(web::scope("/vrp").configure(vrp::vrp_config))
    })
    .bind("127.0.0.1:8753")?
    .run()
    .await
}

fn set_environment() {
    env_logger::init();
    //    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    //    std::env::set_var("RUST_BACKTRACE", "full");
}
