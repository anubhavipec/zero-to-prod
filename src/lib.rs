
use std::net::TcpListener;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;

pub mod configuration;
pub mod startup;
pub mod routes;



async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}



async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!",name)
}



