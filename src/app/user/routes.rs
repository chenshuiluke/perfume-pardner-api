use crate::app::{errors::RegisterUserError, models::user::User};

use super::dto::register_user::RegisterUser;
use super::service;
use actix_web::{web, Responder, Result as ActixResult};
async fn register(json: web::Json<RegisterUser>) -> ActixResult<impl Responder> {
    let result = service::register_user(json).await;

    match result {
        Ok(response) => Ok(web::Json(response)),
        Err(err) => Err(err.into()),
    }
}

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user").route(web::post().to(register)));
}
