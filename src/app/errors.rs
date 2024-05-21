use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError};
use serde::Serialize;

use super::models::user::User;

#[derive(Serialize)]
struct CustomHttpError {
    error: String,
}

#[derive(Debug)]
pub enum RegisterUserError {
    DbError(sqlx::Error),
    UserAlreadyExistsError(User),
    ArgonError(argon2::password_hash::Error),
}

impl From<sqlx::Error> for RegisterUserError {
    fn from(err: sqlx::Error) -> Self {
        Self::DbError(err)
    }
}

impl From<argon2::password_hash::Error> for RegisterUserError {
    fn from(err: argon2::password_hash::Error) -> Self {
        Self::ArgonError(err)
    }
}

impl Error for RegisterUserError {}

impl Display for RegisterUserError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            RegisterUserError::DbError(e) => write!(f, "SQLX db error: {}", e),
            RegisterUserError::UserAlreadyExistsError(user) => {
                write!(
                    f,
                    "User already exists {} {}",
                    user.email.clone(),
                    user.username.clone()
                )
            }
            RegisterUserError::ArgonError(e) => write!(f, "Argon2 error: {}", e),
        }
    }
}

impl Serialize for RegisterUserError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            RegisterUserError::DbError(e) => {
                serializer.serialize_str(format!("SQLX db error: {}", e).as_str())
            }
            RegisterUserError::UserAlreadyExistsError(user) => serializer.serialize_str(
                format!(
                    "User already exists {} {}",
                    user.email.clone(),
                    user.username.clone()
                )
                .as_str(),
            ),
            RegisterUserError::ArgonError(e) => {
                serializer.serialize_str(format!("Argon2 error: {}", e).as_str())
            }
        }
    }
}
impl ResponseError for RegisterUserError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            RegisterUserError::DbError(_) => HttpResponse::InternalServerError().finish(),
            RegisterUserError::UserAlreadyExistsError(_) => {
                HttpResponseBuilder::new(StatusCode::BAD_REQUEST).json({
                    CustomHttpError {
                        error: format!("User with username or email already exists."),
                    }
                })
            }
            RegisterUserError::ArgonError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            RegisterUserError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RegisterUserError::UserAlreadyExistsError(_) => StatusCode::BAD_REQUEST,
            RegisterUserError::ArgonError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
