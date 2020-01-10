use bcrypt::BcryptError;
use diesel::result;
use std::fmt;

#[derive(Debug)]
pub enum AuthError {
    HashError(BcryptError),
    DBError(result::Error),
    PasswordNotMatch(String),
    WrongPassword(String),
    PGConnectionError,
    InternalServerError,
    BadRequest(String),
    Unauthorized,
}

impl From<BcryptError> for AuthError {
    fn from(error: BcryptError) -> Self {
        AuthError::HashError(error)
    }
}

use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;
use uuid::parser::ParseError;

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthError::HashError(error) => write!(f, "{}", error),
            AuthError::DBError(error) => write!(f, "{}", error),
            AuthError::PasswordNotMatch(error) => write!(f, "{}", error),
            AuthError::WrongPassword(error) => write!(f, "{}", error),
            AuthError::PGConnectionError => write!(f, "error obtaining a db connection"),
            AuthError::InternalServerError => write!(f, "InternalServerError"),
            AuthError::BadRequest(message) => write!(f, "{}", message),
            AuthError::Unauthorized => write!(f, "Unauthorized")
        }
    }
}

use serde_json::error::Error as JsonError;
impl From<JsonError> for AuthError {
    fn from(error: JsonError) -> Self {
        AuthError::InternalServerError
    }
}

use actix_http::error::Error as ActixError;

impl From<ActixError> for AuthError {
    fn from(error: ActixError) -> Self {
        AuthError::InternalServerError
    }
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            AuthError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            AuthError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            AuthError::HashError(ref error) => HttpResponse::BadRequest().json("hash error"),
            AuthError::DBError(ref error) => HttpResponse::InternalServerError().json("db error"),
            AuthError::PasswordNotMatch(ref message) => HttpResponse::BadRequest().json(message),
            AuthError::WrongPassword(ref message) => HttpResponse::BadRequest().json(message),
            AuthError::PGConnectionError => HttpResponse::InternalServerError().json("pg error")
        }
    }
}

impl From<ParseError> for AuthError {
    fn from(_: ParseError) -> AuthError {
        AuthError::BadRequest("Invalid UUID".into())
    }
}

impl From<DBError> for AuthError {
    fn from(error: DBError) -> AuthError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return AuthError::BadRequest(message);
                }
                AuthError::InternalServerError
            }
            _ => AuthError::InternalServerError,
        }
    }
}

