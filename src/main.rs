mod app;
mod db;
use actix_web::{get, web, App, HttpServer, Responder, Result as ActixResult};
use app::fragrantica_data;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init_db_pool().await.unwrap();
    let pool = db::DB_POOL.get().expect("Could not get db pool");
    let fragrance_query = sqlx::query!("SELECT name FROM fragrance")
        .fetch_all(pool)
        .await;
    let fragrances = match fragrance_query {
        Ok(fragrances) => fragrances,

        Err(err) => {
            vec![]
        }
    };
    println!("Loading fragrances into db");
    app::fragrance::service::load_fragrances_from_json_to_db().await;
    println!("Loaded fragrances");
    match HttpServer::new(|| {
        App::new()
            .configure(app::fragrance::routes::route_config)
            .configure(app::user::routes::route_config)
    })
    .bind(("0.0.0.0", 8080))
    {
        Ok(server) => {
            println!("Running API");
            server.run().await
        }
        Err(err) => {
            println!("Error running API");
            Err(err)
        }
    }
}
