use sqlx::{PgPool, Error};
use crate::db::models::Target;

pub async fn get_targets(pool: &PgPool) -> Result<Vec<Target>, Error> {
    sqlx::query_as::<_, Target>(
        "SELECT * FROM targets"
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        println!("Failed to get targets: {}", err);
        err
    })
}

pub async fn get_mock_targets() -> Result<Vec<Target>, Error> {
    let url = std::fs::read_to_string("sample_scrape_result/url.txt")
        .expect("Failed to read url");
    let job_title = std::fs::read_to_string("sample_scrape_result/job_title.txt")
        .expect("Failed to read job title");
    let job_location = std::fs::read_to_string("sample_scrape_result/job_location.txt")
        .expect("Failed to read job location");

    Ok(vec![Target {
        id: 1,
        url: url.trim().to_string(),
        job_title: job_title.trim().to_string(),
        job_location: job_location.trim().to_string(),
    }])
}