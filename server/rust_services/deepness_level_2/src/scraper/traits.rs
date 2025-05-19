use anyhow::Result;

#[async_trait::async_trait]
pub trait Scraper {
    async fn scrape(&self, identifier: &str) -> Result<()>;
}
