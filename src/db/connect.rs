use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use crate::aws_secrets_manager::get_secret::get_secret;
use crate::db::models::DbSecrets;

pub async fn establish_connection(is_local: bool) -> PgPool {
    let database_url = if is_local {
        use dotenvy::dotenv;
        dotenv()
            .ok();
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in local environment")
    } else {
        let aws_secret_name = env::var("AWS_DB_SECRETS_NAME")
            .expect("AWS_DB_SECRETS_NAME must be set");
        
        let secrets: DbSecrets = serde_json::from_str(
            &get_secret(&aws_secret_name, None, None)
            .await
            .expect("Failed to get secret")
        )
            .expect("Failed to parse secrets JSON");

        format!("postgres://{}:{}@{}:{}/{}", 
            secrets.username, 
            secrets.password, 
            secrets.host, 
            secrets.port, 
            secrets.dbname
        )
    };

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}