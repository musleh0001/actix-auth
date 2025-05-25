use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn init_db_pool(url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
        .expect("Failed to create pool")
}
