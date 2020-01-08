pub mod authentication;
pub mod products;
pub mod register;

use actix_web::web;
use actix_web::HttpResponse;
use crate::db_connection::{ PgPool, PgPooledConnection };

// Because I'm using this function a lot, 
// I'm including it in the mod file accessible to all handlers.
pub fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
    .get()
    .map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}
