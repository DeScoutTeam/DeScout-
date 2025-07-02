//! This module is responsible for processing raw HTML content, specifically collected tweet HTML,
//! and transforming it into structured `Airdrop` data. It extracts relevant information
//! such as tweet ID, text, author, creation date, mentioned users, and links.

use crate::scraper::user_profile::Airdrop;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use scraper::{ElementRef, Html, Selector, node::Node}; // `node::Node` is not used in the provided code
use std::collections::HashSet;

/// Extracts a tweet ID from a given URL href string.
///
/// This function looks for the "/status/" segment in the URL and attempts to
/// extract the numeric ID that follows it.
///
/// # Arguments
/// * `href` - A string slice representing the `href` attribute of a link.
///
/// # Returns
/// `Option<String>`: The extracted tweet ID as a `String` if found, otherwise `None`.
fn extract_tweet_id_from_href(href: &str) -> Option<String> {
    href.split("/status/")
        .nth(1)
        .map(|s| s.split('/').next().unwrap_or(s).to_string())
}

/// Retrieves the value of a specified HTML attribute from an `ElementRef`.
///
/// # Arguments
/// * `element` - A reference to the `ElementRef` from which to get the attribute.
/// * `attr_name` - The name of the attribute to retrieve (e.g., "href", "datetime").
///
/// # Returns
/// `Option<String>`: The attribute's value as a `String` if the attribute exists, otherwise `None`.
fn get_attribute(element: &ElementRef, attr_name: &str) -> Option<String> {
    element.value().attr(attr_name).map(String::from)
}

/// Processes a `HashSet` of raw HTML strings (representing tweets) into a `Vec` of `Airdrop` structs.
///
/// This function iterates through each HTML string, parses it, and extracts structured
/// data relevant to an airdrop, such as tweet ID, text, author, timestamp, mentioned users,
/// and external links. It expects one of the HTML strings to be an "author_id: <id>" entry
/// to associate the tweets with a specific author.
///
/// # Arguments
/// * `html_set` - A reference to a `HashSet` containing raw HTML strings of tweets
///   and one special string indicating the author ID (e.g., "author_id: 12345").
///
/// # Returns
/// `Result<Vec<Airdrop>>`: A `Result` containing a vector of `Airdrop` structs on success,
/// or an `anyhow::Error` if the author ID is not found or parsing fails.
///
/// # Errors
/// * Returns an error if the "author_id: " entry is missing from the `html_set`.
/// * Returns an error if CSS selectors cannot be parsed.
/// * Logs a warning and skips an HTML item if a tweet ID cannot be extracted.
pub fn process_html_set_to_airdrops(html_set: &HashSet<String>) -> Result<Vec<Airdrop>> {
    let mut airdrops = Vec::new();
    let mut extracted_author_id: Option<String> = None;

    // First, try to extract the author_id which is expected to be a special entry in the HashSet.
    for item_html in html_set {
        if item_html.starts_with("author_id: ") {
            extracted_author_id = Some(item_html.trim_start_matches("author_id: ").to_string());
            break;
        }
    }

    let author_id = extracted_author_id
        .ok_or_else(|| anyhow!("author_id entry not found in the provided HashSet"))?;

    // Define CSS selectors for extracting tweet information.
    let permalink_selector = Selector::parse("a[href*='/status/']")
        .map_err(|e| anyhow!("Failed to parse permalink_selector: {}", e))?;
    let text_selector = Selector::parse("div[data-testid='tweetText']")
        .map_err(|e| anyhow!("Failed to parse text_selector: {}", e))?;
    let time_selector = Selector::parse("time[datetime]")
        .map_err(|e| anyhow!("Failed to parse time_selector: {}", e))?;
    let link_in_text_selector = Selector::parse("a[href]")
        .map_err(|e| anyhow!("Failed to parse link_in_text_selector: {}", e))?;

    // Iterate through each HTML string in the set to parse tweet data.
    for item_html in html_set {
        // Skip the author_id entry itself, as it's not a tweet HTML.
        if item_html.starts_with("author_id: ") {
            continue;
        }

        let document = Html::parse_document(item_html);

        let mut tweet_id_opt: Option<String> = None;
        let mut created_at_opt: Option<DateTime<Utc>> = None;

        // Attempt to extract tweet ID and creation time from the <time> element and its parent.
        if let Some(time_element) = document.select(&time_selector).next() {
            if let Some(datetime_str) = get_attribute(&time_element, "datetime") {
                created_at_opt = DateTime::parse_from_rfc3339(&datetime_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .ok();
            }

            // Sometimes the permalink is the parent <a> of the <time> element
            if let Some(parent_node) = time_element.parent() {
                if let Some(parent_link_element) = ElementRef::wrap(parent_node) {
                    if parent_link_element.value().name() == "a" {
                        if let Some(href) = get_attribute(&parent_link_element, "href") {
                            if let Some(id) = extract_tweet_id_from_href(&href) {
                                tweet_id_opt = Some(id);
                            }
                        }
                    }
                }
            }
        }

        // If tweet ID was not found via the time element's parent, try other permalinks.
        if tweet_id_opt.is_none() {
            for link_el in document.select(&permalink_selector) {
                if let Some(href) = get_attribute(&link_el, "href") {
                    // Filter out links that are just for photos/videos within a tweet,
                    // focusing on the main tweet permalink.
                    if !href.contains("/photo/") && !href.contains("/video/") {
                        if let Some(id) = extract_tweet_id_from_href(&href) {
                            tweet_id_opt = Some(id);
                            break; // Found the tweet ID, no need to check other links.
                        }
                    }
                }
            }
        }
        
        let tweet_id = match tweet_id_opt {
            Some(id) if !id.is_empty() => id,
            _ => {
                // If tweet ID cannot be determined, log a warning and skip this item.
                eprintln!(
                    "Warning: Skipping HTML item due to missing or empty tweet ID. Author: {}. HTML snippet (first 100 chars): {:.100}",
                    author_id, item_html
                );
                continue;
            }
        };

        // Extract the main text content of the tweet.
        let text_content: Option<String> = document
            .select(&text_selector)
            .next()
            .map(|el| {
                el.text().collect::<String>().trim().to_owned()
            })
            .filter(|s| !s.is_empty()); // Filter out empty strings

        let mut mentioned_users: Vec<String> = Vec::new();
        let mut links_in_tweet: Vec<String> = Vec::new();

        // If a text container is found, extract mentioned users and links within the text.
        if let Some(text_container_el) = document.select(&text_selector).next() {
            if let Some(ref text_val) = text_content {
                // Simple extraction of mentioned users by looking for words starting with '@'.
                mentioned_users = text_val
                    .split_whitespace()
                    .filter(|w| w.starts_with('@'))
                    .map(|w| {
                        w.trim_matches(|c: char| !c.is_alphanumeric() && c != '@')
                            .to_string()
                    })
                    .filter(|s| !s.is_empty() && s != "@")
                    .collect();
            }

            // Extract all hrefs from <a> tags within the tweet text.
            for link_node in text_container_el.select(&link_in_text_selector) {
                if let Some(href) = get_attribute(&link_node, "href") {
                    // Filter for valid HTTP/HTTPS links and exclude links to the tweet itself or hashtags.
                    if (href.starts_with("http://") || href.starts_with("https://"))
                        && !href.contains(&format!("/status/{}", tweet_id))
                        && !href.starts_with('#')
                    {
                        links_in_tweet.push(href);
                    }
                }
            }
            links_in_tweet.dedup(); // Remove duplicate links.
        }

        // Construct the Airdrop struct and add it to the vector.
        airdrops.push(Airdrop {
            tweetId: tweet_id,
            text: text_content,
            authorId: Some(author_id.clone()),
            createdAt: created_at_opt,
            savedAt: Utc::now(), // Timestamp when the Airdrop was processed.
            deepness: 2, // Hardcoded deepness; consider making this dynamic or configurable.
            keywords: Vec::new(), // Keywords will likely be populated by further AI analysis.
            tokenName: None, // Token name will likely be populated by further AI analysis.
            mentionedUsers: mentioned_users,
            links: links_in_tweet,
        });
    }

    Ok(airdrops)
}