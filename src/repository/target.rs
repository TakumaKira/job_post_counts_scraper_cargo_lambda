use sqlx::PgPool;
use crate::db::models::Target;

pub async fn get_targets(pool: &PgPool) -> anyhow::Result<Vec<Target>> {
    let targets = sqlx::query_as::<_, Target>(
        "SELECT * FROM targets"
    )
    .fetch_all(pool)
    .await?;

    Ok(targets)
}

pub async fn get_mock_targets() -> anyhow::Result<Vec<Target>> {
    let url = std::fs::read_to_string("sample_scrape_result/url.txt")?;
    let job_title = std::fs::read_to_string("sample_scrape_result/job_title.txt")?;
    let job_location = std::fs::read_to_string("sample_scrape_result/job_location.txt")?;

    Ok(vec![Target {
        id: 1,
        url: url.trim().to_string(),
        job_title: job_title.trim().to_string(),
        job_location: job_location.trim().to_string(),
    }])
}