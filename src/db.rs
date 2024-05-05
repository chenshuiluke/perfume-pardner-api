use once_cell::sync::OnceCell;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;

pub static DB_POOL: OnceCell<PgPool> = OnceCell::new();
pub async fn init_db_pool() -> Result<(), sqlx::Error> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;
    DB_POOL.set(pool);
    Ok(())
}
