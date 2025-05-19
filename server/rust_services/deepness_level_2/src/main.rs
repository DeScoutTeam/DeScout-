mod config;
mod db;
mod scraper;
mod utils;
mod errors;

use anyhow::Result;
use scraper::user_profile::UserProfileScraper;

fn main() -> Result<()> {
    let scraper = UserProfileScraper::new()?;

    scraper.scrape_user_posts("https://x.com/solana", "author_id")?;

    Ok(())
}

