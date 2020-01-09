pub mod schema;
pub mod db_connection;
pub mod models;
pub mod handlers;
pub mod errors;
pub mod utils;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use] 
extern crate serde_derive;

extern crate actix;
extern crate actix_cors;
extern crate actix_web;
extern crate bcrypt;
extern crate jsonwebtoken as jwt;

#[macro_use]
extern crate dotenv_codegen;

extern crate log;
extern crate env_logger;

extern crate actix_http;

use actix_web::{App, HttpServer, web};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::{cors, Logger};
use chrono::Duration;
use db_connection::establish_connection;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let sys = actix::System::new("mystore");

    HttpServer::new(
    move || App::new()
        .wrap(Logger::default())
        .wrap(
            IdentityService::new(
                CookieIdentityPolicy::new(dotenv!("SECRET_KEY").as_bytes())
                    .domain(dotenv!("HOST"))
                    .name("jwt")
                    .path("/")
                    .max_age(Duration::days(1).num_seconds())
                    .secure(dotenv!("COOKIE_SECURE").parse().unwrap())
            )
        )
        .data(establish_connection())
        .service(
            web::resource("/products")
                .route(web::get().to(handlers::products::index))
                .route(web::post().to(handlers::products::create))
        )
        .service(
            web::resource("/products/{id}")
                .route(web::get().to(handlers::products::show))
                .route(web::delete().to(handlers::products::destroy))
                .route(web::patch().to(handlers::products::update))
        )
        .service(
            web::resource("/register")
                .route(web::post().to(handlers::register::register))
        )
        .service(
            web::resource("/auth")
                .route(web::post().to(handlers::authentication::login))
                .route(web::delete().to(handlers::authentication::logout))
        )
    )
    .bind("0.0.0.0:8088").unwrap()
    .start();

    println!("Started http server: 0.0.0.0:8088");
    let _ = sys.run();
}
