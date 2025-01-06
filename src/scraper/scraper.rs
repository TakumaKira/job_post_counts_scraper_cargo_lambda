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
    pub fn new(
        scrape_service_endpoint_url: &str,
        scrape_service_api_key: &str,
      ) -> Self {
        Self {
            scrape_service_endpoint_url: scrape_service_endpoint_url.to_string(),
            scrape_service_api_key: scrape_service_api_key.to_string(),
        }
    }

    pub async fn scrape_job_post_counts(&self, target_url: &str) -> Result<ScrapedResult, Error> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(900))
            .build()?;
        
        let request = client.get(&self.scrape_service_endpoint_url)
            .query(&[
                ("api_key", &self.scrape_service_api_key), 
                ("url", &target_url.to_string())
            ]);

        match tokio::time::timeout(
            std::time::Duration::from_secs(90),
            async {
                println!("Sending scrape request for {}", target_url);
                let result = request.send().await;
                result
            }
        ).await {
            Ok(Ok(resp)) => {
                if !resp.status().is_success() {
                    println!("Error: Failed to scrape {} with status {}", target_url, resp.status());
                    return Err(resp.error_for_status().unwrap_err());
                }
                
                println!("Successfully got scrape response for {}", target_url);
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
        let scraped_date = std::fs::read_to_string("sample_scrape_result/headers_date.txt")
            .expect("Failed to read mock scraped date");
        let scraped_html = std::fs::read_to_string("sample_scrape_result/text.txt")
            .expect("Failed to read mock scraped html");
                
        Ok(ScrapedResult { scraped_date, scraped_html })
    }
}
