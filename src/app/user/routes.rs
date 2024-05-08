use super::dto::register_user::RegisterUser;
use super::service;
use actix_web::{web, Responder, Result as ActixResult};
async fn register(json: web::Json<RegisterUser>) -> ActixResult<impl Responder> {
    service::register_user(json);
    Ok(web::Json({}))
}

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user").route(web::post().to(register)));
}
