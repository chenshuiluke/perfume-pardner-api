use crate::db;
use actix_web::{get, web, App, HttpServer, Responder, Result as ActixResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FragranceDbRecord {
    id: i32,
    name: String,
    designer: String,
    release_year: i32,
    thumbnail: String,
    fragrantica_id: i32,
    fragrantica_url: String,
}

#[derive(Deserialize)]
struct FragranceSearch {
    query: String,
}

async fn get_fragrances(search: web::Query<FragranceSearch>) -> ActixResult<impl Responder> {
    let pool = db::DB_POOL.get().expect("Could not get db pool");
    let fragrance_query = sqlx::query_as!(
        FragranceDbRecord,
        "SELECT * FROM fragrance WHERE LOWER(name) LIKE $1 OR LOWER(designer) LIKE $1",
        search.query
    )
    .fetch_all(pool)
    .await;
    let fragrances = match fragrance_query {
        Ok(fragrances) => fragrances,

        Err(err) => {
            vec![]
        }
    };
    Ok(web::Json(fragrances))
}

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/fragrances").route(web::get().to(get_fragrances)));
}
