use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};

#[derive(Debug)]
pub enum RegisterUserError {
    DbError(sqlx::Error),
}

impl From<sqlx::Error> for RegisterUserError {
    fn from(err: sqlx::Error) -> Self {
        Self::DbError(err)
    }
}

impl Error for RegisterUserError {}

impl Display for RegisterUserError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::DbError(e) => write!(f, "SQLX db error: {}", e),
        }
    }
}
