use crate::utils::jwt::create_token;
use actix_identity::{Identity};
use actix_web::web;
use actix_web::HttpResponse;

use crate::db_connection::PgPool;
use crate::errors::AuthError;
use crate::handlers::pg_pool_handler;
use crate::models::user::AuthUser;

pub fn login(
    auth_user: web::Json<AuthUser>,
    id: Identity,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    let user = auth_user.login(&pg_pool).map_err(|e| match e {
        AuthError::DBError(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().json(e.to_string())
        }
        _ => HttpResponse::InternalServerError().json(e.to_string()),
    })?;

    let token = create_token(user.id, &user.email, &user.username)?;

    id.remember(token);
    let response = HttpResponse::Ok().json(user);
    Ok(response)
}

pub fn logout(id: Identity) -> Result<HttpResponse, HttpResponse> {
    id.forget();
    Ok(HttpResponse::Ok().json("success"))
}
