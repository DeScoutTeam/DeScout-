// src/scraper/user_profile.rs

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use headless_chrome::{Browser, Tab, browser::tab::element::Element};
use serde::{Serialize, Deserialize};
use std::{fs::File, io::Write};

// Your Airdrop struct matches your Mongoose model for posts
#[derive(Debug, Serialize, Deserialize)]
pub struct Airdrop {
    pub tweetId: String,
    pub text: Option<String>,
    pub authorId: Option<String>,
    pub createdAt: Option<DateTime<Utc>>,
    pub savedAt: DateTime<Utc>,
    pub deepness: u32,
    pub keywords: Vec<String>,
    pub tokenName: Option<String>,
    pub mentionedUsers: Vec<String>,
    pub links: Vec<String>,
}

// Helper to extract attribute value from Element
fn get_attribute_value(element: &Element, attr_name: &str) -> Result<Option<String>> {
    if let Some(attrs) = element.get_attributes()? {
        for pair in attrs.chunks(2) {
            if pair[0] == attr_name {
                return Ok(Some(pair[1].clone()));
            }
        }
    }
    Ok(None)
}

pub struct UserProfileScraper {
    browser: Browser,
}

impl UserProfileScraper {
    pub fn new() -> Result<Self> {
        let browser = Browser::default()?;
        Ok(Self { browser })
    }

    /// Scrapes recent tweets from a user profile URL (e.g. https://twitter.com/username)
    /// Saves scraped posts into a JSON file.
    pub fn scrape_user_posts(&self, profile_url: &str, author_id: &str) -> Result<()> {
        let tab = self.browser.new_tab()?;
        tab.navigate_to(profile_url)?;
        tab.wait_until_navigated()?;

        // Wait a bit for tweets to load (improve with explicit waits if needed)
        std::thread::sleep(std::time::Duration::from_secs(5));

        // Select tweet elements (simplified selector, adjust if twitter markup changes)
        let tweet_elements = tab.find_elements("article div[data-testid='tweet']")?;

        let mut posts = Vec::new();

        for tweet in tweet_elements.into_iter() {
            // Extract tweet id (from data-testid or link href)
            let tweet_id = if let Ok(link) = tweet.find_element("a[href*='/status/']") {
                get_attribute_value(&link, "href")?
                    .and_then(|href| href.split("/status/").nth(1).map(|s| s.to_string()))
                    .unwrap_or_default()
            } else {
                continue;
            };

            // Extract tweet text content
            let text = tweet.get_inner_text().ok();

            // Extract createdAt from <time datetime="...">
            let created_at: Option<DateTime<Utc>> = if let Ok(time_el) = tweet.find_element("time") {
                if let Ok(Some(datetime_str)) = get_attribute_value(&time_el, "datetime") {
                    DateTime::parse_from_rfc3339(&datetime_str)
                        .map(|dt| dt.with_timezone(&Utc))
                        .ok()
                } else {
                    None
                }
            } else {
                None
            };

            // Extract mentioned users (simplified, collect all @usernames in text)
            let mentioned_users = text
                .as_ref()
                .map(|txt| {
                    txt.split_whitespace()
                        .filter(|w| w.starts_with('@'))
                        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '@').to_string())
                        .collect()
                })
                .unwrap_or_default();

            // Extract links (simplified: find <a> hrefs inside tweet)
            let links: Vec<String> = tweet
                .find_elements("a")
                .map(|links| {
                    links.into_iter().filter_map(|link| {
                        get_attribute_value(&link, "href").unwrap_or(None)
                    }).collect()
                })
                .unwrap_or_default();

            let post = Airdrop {
                tweetId: tweet_id,
                text,
                authorId: Some(author_id.to_string()),
                createdAt: created_at,
                savedAt: Utc::now(),
                deepness: 2,
                keywords: vec![],    // Implement your keyword extraction later
                tokenName: None,     // Implement your token detection later
                mentionedUsers: mentioned_users,
                links,
            };

            posts.push(post);
        }

        // Save to JSON file
        let file_path = format!("{}_posts.json", author_id);
        let mut file = File::create(&file_path)?;
        let json = serde_json::to_string_pretty(&posts)?;
        file.write_all(json.as_bytes())?;

        println!("Saved {} posts to {}", posts.len(), file_path);

        Ok(())
    }
}
