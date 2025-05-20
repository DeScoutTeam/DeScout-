use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use headless_chrome::{Browser, LaunchOptions, Tab, browser::tab::element::Element};
use serde::{Serialize, Deserialize};
use std::{fs::File, io::Write, sync::Arc};

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
        let options = LaunchOptions::default_builder()
            .headless(false)
            .build()?;

        let browser = Browser::new(options)?;
        Ok(Self { browser })
    }

    pub fn scrape_user_posts(&self, profile_url: &str, author_id: &str) -> Result<()> {
        let tab: Arc<Tab> = self.browser.new_tab()?;
        tab.navigate_to(profile_url)?;
        tab.wait_until_navigated()?;

        println!("Waiting for page to load tweets...");
        std::thread::sleep(std::time::Duration::from_secs(8));

        let tweet_selector = "article[data-testid='tweet']";
        println!("Looking for tweets with selector: {}", tweet_selector);
        let tweet_elements = tab.find_elements(tweet_selector)?;

        println!("Found {} potential tweet elements.", tweet_elements.len());
        let initial_tweet_elements_count = tweet_elements.len();

        if tweet_elements.is_empty() {
            println!("No tweet elements found. The page structure might have changed or content didn't load.");
            return Err(anyhow!("No tweet elements found on the page with selector: {}", tweet_selector));
        }

        let mut posts = Vec::new();

        for (index, tweet) in tweet_elements.iter().enumerate() {
            println!("Processing tweet element #{}", index + 1);

            let permalink_anchor_selector = "a[href*='/status/'][role='link']";
            let tweet_id = match tweet.find_element(permalink_anchor_selector) {
                Ok(link_element) => {
                    get_attribute_value(&link_element, "href")
                        .ok()
                        .flatten()
                        .and_then(|href| href.split("/status/").nth(1).map(|s| s.split('/').next().unwrap_or(s).to_string()))
                        .unwrap_or_else(|| {
                            eprintln!("Could not parse tweet ID from href for element using selector: {}", permalink_anchor_selector);
                            format!("unknown_id_href_parse_{}", Utc::now().timestamp_millis())
                        })
                }
                Err(_e) => {
                    let fallback_selector = "a[href*='/status/']";
                    match tweet.find_element(fallback_selector) {
                        Ok(any_link_element) => {
                             get_attribute_value(&any_link_element, "href")
                                .ok().flatten()
                                .and_then(|href| href.split("/status/").nth(1).map(|s| s.split('/').next().unwrap_or(s).to_string()))
                                .unwrap_or_else(|| {
                                    eprintln!("Could not parse tweet ID from fallback href using selector: {}", fallback_selector);
                                    format!("unknown_id_fallback_parse_{}", Utc::now().timestamp_millis())
                                })
                        },
                        Err(_e_fallback) => {
                            eprintln!("Error finding even a fallback tweet link for ID. Using placeholder ID.");
                            format!("unknown_id_no_link_{}", Utc::now().timestamp_millis())
                        }
                    }
                }
            };

            if tweet_id.starts_with("unknown_id_") {
                 println!("Failed to extract a valid tweet ID for element #{}, using placeholder: {}", index + 1, tweet_id);
            }


            let text_selector = "div[data-testid='tweetText']";
            let text = match tweet.find_element(text_selector) {
                Ok(text_element) => text_element.get_inner_text().ok(),
                Err(_) => {
                    tweet.get_inner_text().ok().map(|full_text| {
                        full_text.lines().filter(|line| !line.trim().is_empty()).collect::<Vec<_>>().join("\n")
                    })
                }
            };

            if text.is_none() || text.as_deref().map_or(true, |s| s.trim().is_empty()) {
                println!("Tweet #{} (ID: {}) has no discernible text content or failed to extract.", index + 1, tweet_id);
            }

            let time_selector = "time[datetime]";
            let created_at_var: Option<DateTime<Utc>> = match tweet.find_element(time_selector) {
                Ok(time_el) => {
                    if let Ok(Some(datetime_str)) = get_attribute_value(&time_el, "datetime") {
                        DateTime::parse_from_rfc3339(&datetime_str)
                            .map(|dt| dt.with_timezone(&Utc))
                            .ok()
                    } else {
                        None
                    }
                }
                Err(_) => {
                    println!("Could not find time element for tweet ID: {}", tweet_id);
                    None
                }
            };

            let mentioned_users_var = text
                .as_ref()
                .map(|txt| {
                    txt.split_whitespace()
                        .filter(|w| w.starts_with('@'))
                        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '@').to_string())
                        .filter(|s| !s.is_empty() && s != "@")
                        .collect()
                })
                .unwrap_or_default();

            let links_selector = "a[href]";
            let links: Vec<String> = tweet
                .find_elements(links_selector)
                .map(|links_vec| {
                    links_vec.into_iter().filter_map(|link_el| {
                        match get_attribute_value(&link_el, "href") {
                            Ok(Some(href_val)) if !href_val.starts_with('#') && !href_val.starts_with("javascript:") => Some(href_val),
                            _ => None,
                        }
                    }).collect()
                })
                .unwrap_or_default();

            let post = Airdrop {
                tweetId: tweet_id.clone(),
                text: text.clone(),
                authorId: Some(author_id.to_string()),
                createdAt: created_at_var,
                savedAt: Utc::now(),
                deepness: 2,
                keywords: vec![],
                tokenName: None,
                mentionedUsers: mentioned_users_var,
                links,
            };
            println!("Scraped post: Tweet ID {}, Text: '{}...', CreatedAt: {:?}",
                post.tweetId,
                post.text.as_deref().unwrap_or("N/A").chars().take(50).collect::<String>().replace("\n", " "),
                post.createdAt
            );
            posts.push(post);
        }

        if posts.is_empty() && initial_tweet_elements_count > 0 {
             println!("Found {} tweet elements but failed to parse any into posts. Check selectors and parsing logic. Also, ensure the page content is as expected (e.g., not a login wall or CAPTCHA).", initial_tweet_elements_count);
        } else if posts.is_empty() {
             println!("No posts were scraped. The profile might be empty, private, or the selectors failed initially.");
        }


        let file_path = format!("{}_posts.json", author_id);
        let mut file = File::create(&file_path)?;
        let json = serde_json::to_string_pretty(&posts)?;
        file.write_all(json.as_bytes())?;

        println!("Saved {} posts to {}", posts.len(), file_path);

        Ok(())
    }
}