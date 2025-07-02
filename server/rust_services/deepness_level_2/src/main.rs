//! This is the main entry point for the application.
//! It orchestrates the web scraping process, from initializing the scraper
//! to processing the collected data.
//!
//! In the future, this module will be enhanced to pull Twitter profiles
//! from a database and distribute them among several concurrent scrapers
//! for efficient and scalable data collection.

// Declare the modules within the current crate.
mod config;      // Configuration settings for the application.
mod db;          // Database interaction utilities (future: to pull profiles, store data).
mod scraper;     // Web scraping components.
mod utils;       // General utility functions.
mod errors;      // Custom error types for the application.
mod processing;  // Data processing and transformation components.

// Import necessary items from declared modules.
use anyhow::Result;
use scraper::user_profile::UserProfileScraper;
use processing::html_processor::process_html_set_to_airdrops;

/// The main function where the application execution begins.
///
/// This function currently demonstrates a single-profile scraping flow:
/// 1. Initializes a `UserProfileScraper`.
/// 2. Defines a hardcoded target profile URL and author ID.
/// 3. Initiates the scraping process to collect raw HTML tweets.
/// 4. Processes the collected HTML into structured `Airdrop` data.
/// 5. Prints a summary of the processed `Airdrop` records.
///
/// **Future Enhancements:**
/// This `main` function is designed to evolve. In upcoming iterations,
/// a dedicated module (likely within `db` or a new `scheduler` module)
/// will be introduced to:
/// - Pull a list of Twitter profiles (e.g., specific user handles or search queries)
///   from a database.
/// - Manage a pool of `UserProfileScraper` instances.
/// - Distribute the scraping tasks to these concurrent scrapers, allowing
///   for parallel processing and significantly faster data acquisition
///   from multiple profiles simultaneously.
/// - Once scraped, the raw HTML or processed `Airdrop` data will then be
///   persisted back into the database for further analysis (e.g., by AI models).
///
/// # Returns
/// `Result<()>`: `Ok(())` if the entire process completes successfully,
/// otherwise an `anyhow::Error` if any step (initialization, scraping, or processing) fails.
fn main() -> Result<()> {
    // 1. Initialize the web scraper.
    let scraper = UserProfileScraper::new()?;

    // 2. Define the target profile for the current single-threaded demonstration.
    // In the future, these values will be dynamically fetched from a database.
    let profile_url = "https://x.com/solana";
    let author_id = "solana_official";

    println!("Starting the scraping process...");
    println!("Target profile: {}\nAuthor ID: {}", profile_url, author_id);

    // 3. Execute the scraping operation.
    match scraper.scrape_user_posts(profile_url, author_id) {
        Ok(html_set) => {
            println!("\nScraping complete. Found {} HTML posts.", html_set.len());
            println!("Processing the scraped content into Airdrop data...");

            // 4. Process the collected raw HTML into structured `Airdrop` data.
            match process_html_set_to_airdrops(&html_set) {
                Ok(airdrops) => {
                    println!("\nProcessed {} Airdrop records:", airdrops.len());
                    // 5. Print out the details of each processed Airdrop.
                    for (i, airdrop) in airdrops.iter().enumerate() {
                        println!("\n{}. Tweet ID: {}", i + 1, airdrop.tweetId);
                        println!("    Text       : {}", airdrop.text.as_deref().unwrap_or("No text"));
                        println!("    Author ID  : {}", airdrop.authorId.as_deref().unwrap_or("Unknown"));
                        println!("    Created At : {:?}", airdrop.createdAt);
                        println!("    Mentions   : {:?}", airdrop.mentionedUsers);
                        println!("    Links      : {:?}", airdrop.links);
                        println!("----------------------------------------------------");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to process HTML into Airdrops: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Scraping failed: {}", e);
        }
    }

    println!("\nDone.");
    Ok(())
}