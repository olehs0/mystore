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
}

impl From<BcryptError> for AuthError {
    fn from(error: BcryptError) -> Self {
        AuthError::HashError(error)
    }
}

impl From<result::Error> for AuthError {
    fn from(error: result::Error) -> Self {
        AuthError::DBError(error)
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthError::HashError(error) => write!(f, "{}", error),
            AuthError::DBError(error) => write!(f, "{}", error),
            AuthError::PasswordNotMatch(error) => write!(f, "{}", error),
            AuthError::WrongPassword(error) => write!(f, "{}", error),
            AuthError::PGConnectionError => write!(f, "error obtaining a db connection"),
        }
    }
}
