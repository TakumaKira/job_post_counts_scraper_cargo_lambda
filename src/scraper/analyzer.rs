use chrono::NaiveDateTime;
use url::Url;
use scraper::{Html, Selector};

pub trait ScrapeResultAnalyzer {
    /// Converts the scraped date string into a NaiveDateTime
    fn convert_scrape_date(&self, date_str: &str) -> Result<NaiveDateTime, chrono::ParseError> {
        NaiveDateTime::parse_from_str(date_str, "%a, %d %b %Y %H:%M:%S GMT")
    }

    /// Retrieves the job count from the scraped HTML
    fn retrieve_count(&self, html: &str, job_title: &str, job_location: &str) -> Result<i32, Box<dyn std::error::Error>>;
}

pub struct GlassdoorScrapeResultAnalyzer;

impl GlassdoorScrapeResultAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Retrieves the page title from the scraped HTML
    fn retrieve_page_title(&self, html: &str) -> Result<String, Box<dyn std::error::Error>> {
        let document = Html::parse_document(html);
        let title_selector = Selector::parse("title").unwrap();
        let title_element = document.select(&title_selector)
            .next()
            .ok_or("No title element found in HTML")?;
        let title = title_element.text().collect::<String>();
        Ok(title)
    }

    fn analyze_page_title(&self, title: &str, job_title: &str, job_location: &str) -> Result<i32, Box<dyn std::error::Error>> {
        // Split the title by spaces to get the first number
        let parts: Vec<&str> = title.split_whitespace().collect();
        
        // First part should be the count
        let count_str = parts.first()
            .ok_or("Title format invalid")?;
            
        // Remove commas and parse as integer
        let count = count_str
            .replace(',', "")
            .parse::<i32>()
            .map_err(|_| "Count not found in title")?;

        // Verify that job title and location are in the title
        if !title.to_lowercase().contains(&job_title.to_lowercase()) {
            return Err("Job title not found in page title".into());
        }
        if !title.to_lowercase().contains(&job_location.to_lowercase()) {
            return Err("Job location not found in page title".into());
        }

        Ok(count)
    }
}

impl ScrapeResultAnalyzer for GlassdoorScrapeResultAnalyzer {
    fn retrieve_count(&self, html: &str, job_title: &str, job_location: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let title = self.retrieve_page_title(html)?;
        let count = self.analyze_page_title(&title, job_title, job_location)?;
        Ok(count)
    }
}

pub fn get_analyzer(url: &str) -> Result<Box<dyn ScrapeResultAnalyzer>, Box<dyn std::error::Error>> {
    let parsed_url = Url::parse(url)?;
    let host = parsed_url.host_str().ok_or("No host in URL")?;

    match host {
        host if host.contains("glassdoor") => Ok(Box::new(GlassdoorScrapeResultAnalyzer::new())),
        _ => Err("No analyzer available for this URL".into()),
    }
} 