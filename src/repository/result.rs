use sqlx::{PgPool, Error};
use crate::db;

pub async fn get_next_result_id(pool: &PgPool) -> Result<i32, Error> {
    sqlx::query_scalar::<_, i32>(
        "SELECT COALESCE(MAX(id), 0) + 1 FROM results"
    )
    .fetch_one(pool)
    .await
    .map_err(|err| {
        println!("Failed to get next result id: {}", err);
        err
    })
}

pub async fn create_results(pool: &PgPool, results: Vec<db::models::Result>) -> Result<(), Error> {
    let mut tx = pool.begin()
        .await
        .expect("Failed to begin transaction");
    for result in results {
        sqlx::query(
            "INSERT INTO results (id, url, job_title, job_location, scrape_date, count) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(result.id)
        .bind(&result.url)
        .bind(&result.job_title)
        .bind(&result.job_location)
        .bind(result.scrape_date)
        .bind(result.count)
        .execute(&mut *tx)
        .await
        .expect(format!("Failed to insert result: {:?}", result).as_str());
    }
    tx.commit()
    .await
    .map_err(|err| {
        println!("Failed to commit transaction: {}", err);
        err
    })
}
