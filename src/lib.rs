use std::net::TcpListener;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;


async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!",name)
}


pub  fn run(listener: TcpListener) -> Result<Server,std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/",web::get().to(greet))
            .route("/greet/{name}",web::get().to(greet))
            .route("/health_check",web::get().to(health_check))
    })
        .listen(listener)?
        .run();
        Ok(server)
}
