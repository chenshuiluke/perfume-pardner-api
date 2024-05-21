use std::error::Error;

use crate::app::errors::RegisterUserError;
use crate::app::models::user::{self, User};
use crate::db;
use actix_web::web;

use super::dto::register_user::RegisterUser;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub async fn register_user(user: web::Json<RegisterUser>) -> Result<user::User, RegisterUserError> {
    let previous_user =
        find_user_by_email_or_username(user.email.clone(), user.username.clone()).await;

    match previous_user {
        Ok(user) => Err(RegisterUserError::UserAlreadyExistsError(user)),
        Err(err) => match err {
            sqlx::error::Error::RowNotFound => return insert_user(user).await,
            _ => Err(RegisterUserError::DbError(err)),
        },
    }
}

pub async fn insert_user(user: web::Json<RegisterUser>) -> Result<User, RegisterUserError> {
    let pool = db::DB_POOL.get().expect("Could not get db pool");

    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(user.password.as_bytes(), &salt)?
        .to_string();

    let _ = sqlx::query!(
        "INSERT INTO user_account (username, email, password) VALUES($1, $2, $3)",
        user.username,
        user.email,
        password_hash
    )
    .execute(pool)
    .await?;
    self::find_user_by_email_or_username(user.email.clone(), user.username.clone())
        .await
        .map_err(|err| -> RegisterUserError { RegisterUserError::DbError(err) })
}

pub async fn find_user_by_email_or_username(
    email: String,
    username: String,
) -> Result<user::User, sqlx::Error> {
    let pool = db::DB_POOL.get().expect("Could not get db pool");
    let user = sqlx::query_as!(
        user::User,
        "SELECT id, username, email, password FROM user_account WHERE email = $1 OR username = $2 LIMIT 1",
        email,
        username
    )
    .fetch_one(pool)
    .await;
    match user {
        Ok(user) => Ok(user),
        Err(err) => Err(err),
    }
}
