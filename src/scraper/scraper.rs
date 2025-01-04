use reqwest::Error;

pub struct ScrapedResult {
    pub scraped_date: String,
    pub scraped_html: String,
}

pub struct Scraper {
    scrape_service_endpoint_url: String,
    scrape_service_api_key: String,
}

impl Scraper {
    pub fn new(scrape_service_endpoint_url: &str, scrape_service_api_key: &str) -> Self {
        Self {
            scrape_service_endpoint_url: scrape_service_endpoint_url.to_string(),
            scrape_service_api_key: scrape_service_api_key.to_string(),
        }
    }

    pub async fn scrape_job_post_counts(&self, target_url: &str) -> Result<ScrapedResult, Error> {
        println!("Scraping job post counts for {}", target_url);
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(900))
            .build()?;
        
        println!("Scrape service endpoint URL: {}", self.scrape_service_endpoint_url);
        println!("Sending request...");

        let request = client.get(&self.scrape_service_endpoint_url)
            .query(&[
                ("api_key", &self.scrape_service_api_key), 
                ("url", &target_url.to_string())
            ]);
            
        println!("Request URL: {}", request.try_clone().unwrap().build()?.url());

        match tokio::time::timeout(
            std::time::Duration::from_secs(900),
            async {
                println!("Starting request for {}", target_url);
                let result = request.send().await;
                println!("Request completed for {}", target_url);
                result
            }
        ).await {
            Ok(Ok(resp)) => {
                println!("Response received with status: {}", resp.status());
                if !resp.status().is_success() {
                    println!("Error: Failed to scrape {} with status {}", target_url, resp.status());
                    return Err(resp.error_for_status().unwrap_err());
                }
                
                let scraped_date = match resp.headers().get("date") {
                    Some(date) => date.to_str().unwrap_or_default().to_string(),
                    None => chrono::Utc::now().to_rfc2822(),
                };
                let scraped_html = resp.text().await?;
                Ok(ScrapedResult { scraped_date, scraped_html })
            },
            Ok(Err(e)) => {
                println!("Request error for {}: {}", target_url, e);
                Err(e)
            },
            Err(elapsed) => {
                println!("Request timed out after {:?} for {}", elapsed, target_url);
                Ok(ScrapedResult {
                    scraped_date: chrono::Utc::now().to_rfc2822(),
                    scraped_html: String::new()
                })
            }
        }
    }

    pub async fn scrape_job_post_counts_dry_run(&self, _target_url: &str) -> Result<ScrapedResult, Error> {
        let scraped_date = include_str!("../../sample_scrape_result/headers_date.txt").to_string();
        let scraped_html = include_str!("../../sample_scrape_result/text.txt").to_string();
                
        Ok(ScrapedResult { scraped_date, scraped_html })
    }
}
