pub mod authentication;
pub mod products;
pub mod register;

use actix_web::web;
use actix_web::{ HttpResponse, HttpRequest};
use crate::db_connection::{ PgPool, PgPooledConnection };

pub fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
    .get()
    .map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}

use actix_web::FromRequest;
use crate::utils::jwt::SlimUser;
use crate::errors::AuthError;
use actix_identity::Identity;
use actix_web::{
    dev::Payload
};


#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

pub type LoggedUser = SlimUser;

impl FromRequest for LoggedUser {
    type Error = AuthError;
    type Future = Result<LoggedUser, AuthError>;
    type Config = ();

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Some(identity) = Identity::from_request(req, pl)?.identity() {
            let user: LoggedUser = serde_json::from_str(&identity)?;
            return Ok(user);
        }
        Err(AuthError::Unauthorized.into())
    }
}

