mod app;
mod db;
use actix_web::{get, web, App, HttpServer, Responder, Result as ActixResult};
use app::fragrantica_data;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

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

#[get("/fragrances")]
async fn get_fragrances() -> ActixResult<impl Responder> {
    let pool = db::connection_builder().await.unwrap();
    let perfume_query = sqlx::query_as!(FragranceDbRecord, "SELECT * FROM perfume")
        .fetch_all(&pool)
        .await;
    let perfumes = match perfume_query {
        Ok(perfumes) => perfumes,

        Err(err) => {
            vec![]
        }
    };
    Ok(web::Json(perfumes))
}
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::connection_builder().await.unwrap();
    let perfume_query = sqlx::query!("SELECT name FROM perfume")
        .fetch_all(&pool)
        .await;
    let perfumes = match perfume_query {
        Ok(perfumes) => perfumes,

        Err(err) => {
            vec![]
        }
    };
    load_fragrances_from_json_to_db(pool).await;
    HttpServer::new(|| App::new().service(get_fragrances).service(greet))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

async fn load_fragrances_from_json_to_db(pool: PgPool) {
    match fragrantica_data::read_from_file("./output.json") {
        Ok(fragrances) => {
            for fragrance in fragrances.results[0].hits.iter() {
                sqlx::query!("INSERT INTO perfume (name, designer, release_year, thumbnail, fragrantica_id, fragrantica_url) VALUES($1, $2, $3, $4, $5, $6)",
                fragrance.naslov,
                fragrance.dizajner,
                fragrance.godina,
                fragrance.thumbnail,
                fragrance.objectID.parse::<i32>().unwrap(),
                fragrance.url.get("EN").unwrap()[0]
                ).execute(&pool).await;
            }
        }
        Err(err) => {
            println!("Error loading fragrances from JSON: {:?}", err)
        }
    };
}
