use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn connection_builder() -> Result<PgPool, sqlx::Error> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgPoolOptions::new().max_connections(5).connect(&url).await
}
