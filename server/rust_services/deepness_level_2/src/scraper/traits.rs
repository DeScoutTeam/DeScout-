//! This module defines traits that establish common interfaces for different web scrapers.

use anyhow::Result;

/// `Scraper` is an asynchronous trait that defines the core behavior for any web scraper.
///
/// Implementors of this trait are expected to perform a scraping operation
/// based on a given identifier.
#[async_trait::async_trait]
pub trait Scraper {
    /// Asynchronously scrapes data using the provided identifier.
    ///
    /// The `identifier` could be a URL, a user ID, a search query, or any
    /// other string that helps the scraper locate and retrieve the desired information.
    ///
    /// # Arguments
    /// * `identifier` - A string slice that uniquely identifies the target for scraping.
    ///
    /// # Returns
    /// `Result<()>`: An `Ok(())` on successful completion of the scraping operation,
    /// or an `anyhow::Error` if an error occurs during scraping.
    async fn scrape(&self, identifier: &str) -> Result<()>;
}