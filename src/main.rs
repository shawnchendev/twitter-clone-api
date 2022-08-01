#[macro_use]
extern crate actix_web;
use actix_web::{get, middleware, post, App, HttpResponse, HttpServer, Responder};
use std::{env, io};

mod constants;
mod response;
mod tweet;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(tweet::create)
            .service(tweet::list)
            .service(tweet::delete)
            .service(tweet::get)
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
