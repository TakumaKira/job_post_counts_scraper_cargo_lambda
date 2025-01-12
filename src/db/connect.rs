use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use crate::aws_secrets_manager::get_secret::get_secret;
use crate::db::models::DbSecrets;

pub async fn establish_connection() -> PgPool {
    // Check if running locally
    let is_local = env::var("LOCAL")
        .is_ok();
    
    let database_url = if is_local {
        use dotenvy::dotenv;
        dotenv()
            .ok();
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in local environment")
    } else {
        let host = env::var("DB_HOST")
            .expect("DB_HOST must be set");
        let port = env::var("DB_PORT")
            .expect("DB_PORT must be set");
        let dbname = env::var("DB_NAME")
            .expect("DB_NAME must be set");
        let aws_secret_name = env::var("AWS_DB_SECRETS_NAME")
            .expect("AWS_DB_SECRETS_NAME must be set");
        
        let secrets: DbSecrets = serde_json::from_str(
            &get_secret(&aws_secret_name, None, None)
            .await
            .expect("Failed to get secret")
        )
            .expect("Failed to parse secrets JSON");

        let encoded_password = utf8_percent_encode(&secrets.password, NON_ALPHANUMERIC).to_string();
        format!("postgres://{}:{}@{}:{}/{}", 
            secrets.username, 
            encoded_password, 
            host, 
            port, 
            dbname
        )
    };

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}
