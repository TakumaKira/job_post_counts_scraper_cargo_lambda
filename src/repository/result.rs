use sqlx::PgPool;
use crate::db::models::Result;

pub async fn get_next_result_id(pool: &PgPool) -> anyhow::Result<i32> {
    let result = sqlx::query_scalar::<_, i32>(
        "SELECT COALESCE(MAX(id), 0) + 1 FROM results"
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn create_results(pool: &PgPool, results: Vec<Result>) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    for result in results {
        sqlx::query(
            "INSERT INTO results (id, url, job_title, job_location, scrape_date, count) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(result.id)
        .bind(result.url)
        .bind(result.job_title)
        .bind(result.job_location)
        .bind(result.scrape_date)
        .bind(result.count)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
}
