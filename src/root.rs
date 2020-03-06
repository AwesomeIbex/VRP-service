use actix_web::{web, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Root {
    links: Vec<&'static str>,
}

#[derive(Serialize)]
struct Service {
    title: &'static str,
    description: &'static str,
    references: Vec<&'static str>,
    tags: Vec<&'static str>,
}

pub fn root_config(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/").route(web::get().to(|| index())))
        .service(web::resource("/service").route(web::get().to(|| service())));
}

pub async fn index() -> HttpResponse {
    let root = Root {
        links: vec![
            "/swagger/openapi",
            "/service",
            "/vrp/calculate",
            "/vrpclass/calculate",
            "/extractor/testpath/test",
        ],
    };
    HttpResponse::Ok().json(root)
}

pub async fn service() -> HttpResponse {
    let root = Service {
        title: "Vehicle Routing Problem",
        description:
            "This service uses the VROOM project and OSRM to solve the Vehicle Routing Problem",
        references: vec![],
        tags: vec![
            "Vehicle Routing Problem",
            "Open Source Routing Machine",
            "Routing",
            "Optimisation",
        ],
    };
    HttpResponse::Ok().json(root)
}
