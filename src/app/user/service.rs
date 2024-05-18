use std::error::Error;

use crate::app::errors::RegisterUserError;
use crate::app::models::user::{self, User};
use crate::db;
use actix_web::web;

use super::dto::register_user::RegisterUser;

pub async fn register_user(user: web::Json<RegisterUser>) -> Result<user::User, RegisterUserError> {
    let previous_user = find_user_by_email(user.email.clone()).await;

    match previous_user {
        Ok(user) => Err(RegisterUserError::UserAlreadyExistsError(user)),
        Err(err) => match err {
            sqlx::error::Error::RowNotFound => {
                return insert_user(user)
                    .await
                    .map_err(|err| -> RegisterUserError { RegisterUserError::DbError(err) })
            }
            _ => Err(RegisterUserError::DbError(err)),
        },
    }
}

pub async fn insert_user(user: web::Json<RegisterUser>) -> Result<User, sqlx::Error> {
    let pool = db::DB_POOL.get().expect("Could not get db pool");
    let _ = sqlx::query!(
        "INSERT INTO user_account (username, email, password) VALUES($1, $2, $3)",
        user.username,
        user.email,
        user.password
    )
    .execute(pool)
    .await?;
    self::find_user_by_email(user.email.clone()).await
}

pub async fn find_user_by_email(email: String) -> Result<user::User, sqlx::Error> {
    let pool = db::DB_POOL.get().expect("Could not get db pool");
    let user = sqlx::query_as!(
        user::User,
        "SELECT id, username, email, password FROM user_account WHERE email = $1 LIMIT 1",
        email
    )
    .fetch_one(pool)
    .await;
    match user {
        Ok(user) => Ok(user),
        Err(err) => Err(err),
    }
}
