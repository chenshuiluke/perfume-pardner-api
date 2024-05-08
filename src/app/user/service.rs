use std::error::Error;

use crate::app::errors::RegisterUserError;
use crate::app::models::user;
use crate::db;
use actix_web::web;

use super::dto::register_user::RegisterUser;

pub async fn register_user(user: web::Json<RegisterUser>) -> Result<user::User, RegisterUserError> {
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

pub async fn find_user_by_email(email: String) -> Result<user::User, RegisterUserError> {
    let pool = db::DB_POOL.get().expect("Could not get db pool");
    let user = sqlx::query_as!(
        user::User,
        "SELECT username, email, password FROM user_account WHERE email = $1",
        email
    )
    .fetch_one(pool)
    .await;
    match user {
        Ok(user) => Ok(user),
        Err(err) => Err(RegisterUserError::DbError(err)),
    }
}
