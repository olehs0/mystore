use chrono::NaiveDateTime;
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    #[serde(skip)]
    pub id: i32,
    pub email: String,
    pub username: String,
    #[serde(skip)]
    pub password: String,
    pub created_at: NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime
}

use bcrypt::{hash, DEFAULT_COST};
use diesel::PgConnection;
use chrono::Local;
use crate::errors::AuthError;

use diesel::RunQueryDsl;

impl User {
    pub fn create(register_user: RegisterUser, connection: &PgConnection) ->
     Result<User, AuthError> {

        Ok(diesel::insert_into(users::table)
            .values(NewUser {
                email: register_user.email,
                username: register_user.username,
                password: Self::hash_password(register_user.password)?,
                created_at: Local::now().naive_local()
            })
            .get_result(connection)?)
    }

    pub fn hash_password(plain: String) -> Result<String, AuthError> {
        Ok(hash(plain, DEFAULT_COST)?)
    }
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub username: String,
    pub password: String,
    pub password_confirmation: String
}

impl RegisterUser {
    pub fn validates(self) ->
     Result<RegisterUser, AuthError> {
         let password_are_equal = self.password == self.password_confirmation;
         let password_not_empty = self.password.len() > 0;
         if password_are_equal && password_not_empty {
             Ok(self)
         } else if !password_are_equal {
             Err(
                 AuthError::PasswordNotMatch(
                     "Password and Password Confirmation does not match".to_string()
                 )
             )
         } else {
             Err(
                 AuthError::WrongPassword(
                     "Wrong Password, check it is not empty".to_string()
                 )
             )
         }
    }
}

#[derive(Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String
}

impl AuthUser {
    pub fn login(&self, connection: &PgConnection) ->
     Result<User, AuthError> {
        use bcrypt::verify;
        use diesel::QueryDsl;
        use diesel::ExpressionMethods;
        use crate::schema::users::dsl::email;

        let mut records =
            users::table
                .filter(email.eq(&self.email))
                .load::<User>(connection)?;

        let user =
            records
                .pop()
                .ok_or(AuthError::DBError(diesel::result::Error::NotFound))?;

        if verify(&self.password, &user.password)? {
            Ok(user)
        } else {
            Err(AuthError::WrongPassword(
                "Wrong password, check again please".to_string()
            ))
        }
    }
}

