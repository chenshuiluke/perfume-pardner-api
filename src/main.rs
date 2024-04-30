mod app;
mod db;
use actix_web::{get, web, App, HttpServer, Responder};
use app::fragrantica_data;
use dotenv::dotenv;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::connection_builder().await.unwrap();
    let perfumes = sqlx::query!("SELECT name FROM perfume")
        .fetch_all(&pool)
        .await;
    match perfumes {
        Ok(rows) => {
            println!("{:?}", rows);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
