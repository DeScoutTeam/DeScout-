//! This module provides the core components for web scraping,
//! including definitions for scraper behaviors and concrete
//! implementations like the `UserProfileScraper`.

/// Defines common behaviors and interfaces for different types of scrapers.
pub mod traits;
/// Implement tools logic, such as login handler
pub mod tools;

/// Implements the logic for scraping data from user profiles on a web platform.
pub mod user_profile;

/// Re-exports the `Scraper` trait for easy access from the parent module.
pub use traits::Scraper;


/// Re-exports the `UserProfileScraper` struct for easy access from the parent module.
pub use user_profile::UserProfileScraper;