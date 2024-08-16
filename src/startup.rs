use std::net::TcpListener;
use std::sync::Arc;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use sqlx::{PgConnection, PgPool};
use crate::{greet, health_check};
use crate::routes::subscribe;

pub  fn run(listener: TcpListener,db_pool: PgPool) -> Result<Server,std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new( move|| {
        App::new()
            .wrap(Logger::default())
            .route("/",web::get().to(greet))
            .route("/greet/{name}",web::get().to(greet))
            .route("/health_check",web::get().to(health_check))
            .route("/subscriptions",web::post().to(subscribe))
            .app_data(db_pool.clone()) // Register the connection as part of app data
    })
        .workers(4)
        .listen(listener)?
        .run();
    Ok(server)
}