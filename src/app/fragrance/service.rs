use crate::app::fragrantica_data;
use crate::db;
pub async fn load_fragrances_from_json_to_db() {
    let pool = db::DB_POOL.get().expect("Could not get db pool");
    match fragrantica_data::read_from_file("./output.json") {
        Ok(fragrances) => {
            let _ = sqlx::query!("TRUNCATE fragrance").execute(pool).await;
            for fragrance in fragrances.results[0].hits.iter() {
                let _ = sqlx::query!("INSERT INTO fragrance (name, designer, release_year, thumbnail, fragrantica_id, fragrantica_url) VALUES($1, $2, $3, $4, $5, $6)",
                fragrance.naslov,
                fragrance.dizajner,
                fragrance.godina,
                fragrance.thumbnail,
                fragrance.objectID.parse::<i32>().unwrap(),
                fragrance.url.get("EN").unwrap()[0]
                ).execute(pool).await;
            }
        }
        Err(err) => {
            println!("Error loading fragrances from JSON: {:?}", err)
        }
    };
}
