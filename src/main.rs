use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use aws_lambda_events::event::eventbridge::EventBridgeEvent;
use job_post_counts_scraper_cargo_lambda::{repository, scraper, db};


/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<EventBridgeEvent>) -> Result<(), Error> {
    // Check if running locally
    let is_local = std::env::var("LOCAL")
        .is_ok();
    if is_local {
        use dotenvy::dotenv;
        dotenv().ok();
    }

    let scrape_service_endpoint_url = std::env::var("SCRAPE_SERVICE_ENDPOINT_URL")
        .expect("SCRAPE_SERVICE_ENDPOINT_URL is not set");
    let scrape_service_api_key = std::env::var("SCRAPE_SERVICE_API_KEY")
        .expect("SCRAPE_SERVICE_API_KEY is not set");
    let scraper = scraper::scraper::Scraper::new(&scrape_service_endpoint_url, &scrape_service_api_key);

    // Check if dry run is requested in the event detail
    let is_dry_run = event.payload.detail
        .as_object()
        .and_then(|obj| obj.get("dry_run"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let pool = db::connect::establish_connection().await;

    let results = scraper::scrape::scrape(&pool, &scraper, is_dry_run).await?;

    repository::result::create_results(&pool, results).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
