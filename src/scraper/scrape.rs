use lambda_runtime::Error;
use sqlx::{Pool, Postgres};
use crate::{db, repository, scraper};

pub async fn scrape(pool: &Pool<Postgres>, scraper: &scraper::scraper::Scraper, is_dry_run: bool) -> Result<Vec::<db::models::Result>, Error> {
    let mut results = Vec::<db::models::Result>::new();

    let next_result_id = repository::result::get_next_result_id(&pool).await?;
    let mut index = 0;

    let targets = if is_dry_run {
        repository::target::get_mock_targets().await?
    } else {
        repository::target::get_targets(&pool).await?
    };

    for target in &targets {
        let scraped_result = if is_dry_run {
            scraper.scrape_job_post_counts_dry_run(&target.url).await
        } else {
            match scraper.scrape_job_post_counts(&target.url).await {
                Ok(result) => Ok(result),
                Err(e) => {
                    eprintln!("Error scraping {}: {}", target.url, e);
                    continue;  // Skip to the next target instead of stopping
                }
            }
        };

        match scraped_result {
            Ok(result) => {
                let analyzer = match scraper::analyzer::get_analyzer(&target.url) {
                    Ok(a) => a,
                    Err(e) => {
                        eprintln!("Error getting analyzer for {}: {}", target.url, e);
                        continue;
                    }
                };

                // Handle all analyzer operations with match
                let count = match analyzer.retrieve_count(&result.scraped_html, &target.job_title, &target.job_location) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Error analyzing count for {}: {}", target.url, e);
                        continue;
                    }
                };

                let scrape_date = match analyzer.convert_scrape_date(&result.scraped_date) {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("Error converting date for {}: {}", target.url, e);
                        continue;
                    }
                };

                let new_result = db::models::Result {
                    id: next_result_id + index as i32,
                    url: target.url.clone(),
                    job_title: target.job_title.clone(),
                    job_location: target.job_location.clone(),
                    scrape_date,
                    count,
                };
                results.push(new_result);
                index += 1;
            },
            Err(e) => eprintln!("Error scraping {}: {}", target.url, e),
        }
    }

    Ok(results)
}