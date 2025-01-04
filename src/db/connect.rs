use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use dotenvy::dotenv;

pub async fn establish_connection() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}