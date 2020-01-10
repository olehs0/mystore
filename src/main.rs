pub mod db_connection;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod schema;
pub mod utils;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

extern crate bcrypt;
extern crate jsonwebtoken as jwt;

#[macro_use]
extern crate dotenv_codegen;

use actix;
use listenfd::ListenFd;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use chrono::Duration;
use db_connection::establish_connection;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let sys = actix::System::new("mystore");
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::new().supports_credentials())
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(dotenv!("SECRET_KEY").as_bytes())
                    .domain(dotenv!("HOST"))
                    .name("jwt")
                    .path("/")
                    .max_age(Duration::days(1).num_seconds())
                    .secure(dotenv!("COOKIE_SECURE").parse().unwrap()),
            ))
            .data(establish_connection())
            .service(
                web::resource("/products")
                    .route(web::get().to(handlers::products::index))
                    .route(web::post().to(handlers::products::create)),
            )
            .service(
                web::resource("/products/{id}")
                    .route(web::get().to(handlers::products::show))
                    .route(web::delete().to(handlers::products::destroy))
                    .route(web::patch().to(handlers::products::update)),
            )
            .service(web::resource("/register").route(web::post().to(handlers::register::register)))
            .service(
                web::resource("/auth")
                    .route(web::post().to(handlers::authentication::login))
                    .route(web::delete().to(handlers::authentication::logout)),
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("0.0.0.0:8088").unwrap()
    };

    server.start();

    println!("Started http server: 0.0.0.0:8088");
    let _ = sys.run();
}
