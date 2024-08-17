use actix_web::{HttpRequest, HttpResponse, Responder};

pub mod configuration;
pub mod startup;
pub mod routes;
pub mod telemetry;


async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}



async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!",name)
}



