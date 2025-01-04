use aws_lambda_events::event::eventbridge::EventBridgeEvent;
use job_post_counts_scraper_cargo_lambda::{repository, scraper::scraper::Scraper, db};
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use dotenvy::dotenv;
use job_post_counts_scraper_cargo_lambda::scraper::analyzer;


/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<EventBridgeEvent>) -> Result<(), Error> {
    dotenv().ok();
    let scrape_service_endpoint_url = std::env::var("SCRAPE_SERVICE_ENDPOINT_URL").unwrap();
    let scrape_service_api_key = std::env::var("SCRAPE_SERVICE_API_KEY").unwrap();
    let scraper = Scraper::new(&scrape_service_endpoint_url, &scrape_service_api_key);

    // Check if dry run is requested in the event detail
    let is_dry_run = event.payload.detail
        .as_object()
        .and_then(|obj| obj.get("dry_run"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let pool = db::connect::establish_connection().await;

    let targets = if is_dry_run {
        repository::target::get_mock_targets().await?
    } else {
        repository::target::get_targets(&pool).await?
    };

    let next_result_id = repository::result::get_next_result_id(&pool).await?;
    let mut index = 0;

    let mut new_results = Vec::<db::models::Result>::new();

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
                let analyzer = match analyzer::get_analyzer(&target.url) {
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
                new_results.push(new_result);
                index += 1;
            },
            Err(e) => eprintln!("Error scraping {}: {}", target.url, e),
        }
    }

    repository::result::create_results(&pool, new_results).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
