use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use aws_lambda_events::event::eventbridge::EventBridgeEvent;
use job_post_counts_scraper_cargo_lambda::{aws_secrets_manager::get_secret::get_secret, db, repository, scraper};


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
    let scrape_service_api_key = if is_local {
        std::env::var("SCRAPE_SERVICE_API_KEY")
            .expect("SCRAPE_SERVICE_API_KEY is not set")
    } else {
        let aws_secret_name = std::env::var("AWS_API_KEY_SECRETS_NAME")
            .expect("AWS_API_KEY_SECRETS_NAME must be set");
        
        let secrets: serde_json::Value = serde_json::from_str(
            &get_secret(&aws_secret_name, None, None)
            .await
            .expect("Failed to get secret")
        )
            .expect("Failed to parse secrets JSON");
        secrets["SCRAPE_OPS_API_KEY"]
            .as_str()
            .expect("SCRAPE_OPS_API_KEY is not set")
            .to_string()
    };
    let scraper = scraper::scraper::Scraper::new(&scrape_service_endpoint_url, &scrape_service_api_key);

    // Check if dry run is requested in the event detail
    let is_dry_run = event.payload.detail
        .as_object()
        .and_then(|obj| obj.get("dry_run"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let pool = db::connect::establish_connection(is_local).await;

    let results = scraper::scrape::scrape(&pool, &scraper, is_dry_run).await?;

    repository::result::create_results(&pool, results).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
