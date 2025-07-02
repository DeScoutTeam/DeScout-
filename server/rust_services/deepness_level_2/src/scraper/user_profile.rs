//! This module provides the `UserProfileScraper` responsible for
//! navigating to user profiles on a social media platform and
//! extracting tweet HTML content by dynamically scrolling.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use headless_chrome::{
    Browser, Tab,
    protocol::cdp::Page,
};
use serde::{Serialize, Deserialize};
use std::{
    collections::HashSet,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

// Import the new LoginHandler module
use crate::scraper::tools::login_handler::LoginHandler;

/// Represents an Airdrop event, typically a tweet containing
/// information about a cryptocurrency airdrop.
#[derive(Debug, Serialize, Deserialize, Clone)]
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

/// `UserProfileScraper` is responsible for automating web browser
/// interactions to scrape information from user profiles.
/// It uses `headless_chrome` to control a Chrome browser instance.
pub struct UserProfileScraper {
    browser: Browser,
    user_data_dir: PathBuf,
}

impl UserProfileScraper {
    /// Creates a new `UserProfileScraper` instance.
    ///
    /// This constructor launches a new headless (or headful, based on configuration)
    /// Chrome browser instance and sets up a user data directory for
    /// persistent Browse data like cookies and local storage.
    ///
    /// # Returns
    /// `Result<Self>`: A `Result` indicating success (`UserProfileScraper`)
    /// or failure (an `anyhow::Error`).
    ///
    /// # Errors
    /// Returns an error if the browser cannot be launched or the user data
    /// directory cannot be created.
    pub fn new() -> Result<Self> {
        let run_headless_hardcoded = false; // Hardcoded for development; consider making this configurable.
        let user_data_dir_path = PathBuf::from("./chrome_scraper_profile");

        if !user_data_dir_path.exists() {
            fs::create_dir_all(&user_data_dir_path)?;
            println!("Created user data directory: {:?}", user_data_dir_path);
        } else {
            println!("Using existing user data directory: {:?}", user_data_dir_path);
        }

        let mut launch_options_builder = headless_chrome::LaunchOptions::default_builder();
        launch_options_builder.headless(run_headless_hardcoded);
        
        // Configure browser options to mimic a typical user.
        launch_options_builder.user_data_dir(Some(user_data_dir_path.clone()));
        launch_options_builder.args(vec![std::ffi::OsStr::new("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")]);
        launch_options_builder.window_size(Some((1920, 1080)));
        launch_options_builder.args(vec![std::ffi::OsStr::new("--disable-blink-features=AutomationControlled")]);
        
        let options = launch_options_builder.build()?;
        let browser = Browser::new(options)?;
        Ok(Self { browser, user_data_dir: user_data_dir_path })
    }

    /// Scrapes tweet HTML content from a user's profile page.
    ///
    /// This function navigates to the specified user profile URL, attempts to
    /// load existing session cookies, and then iteratively scrolls down the
    /// page to load and collect the HTML of tweets. It handles potential
    /// login requirements using the `LoginHandler`.
    ///
    /// # Arguments
    /// * `profile_url` - The URL of the user's profile page to scrape.
    /// * `author_id` - The ID of the author whose tweets are being scraped.
    ///
    /// # Returns
    /// `Result<HashSet<String>>`: A `Result` containing a `HashSet` of unique
    /// tweet HTML strings if successful, or an `anyhow::Error` on failure.
    /// The `HashSet` will also include a string in the format "author_id: <id>".
    ///
    /// # Errors
    /// Returns an error if navigation fails, elements cannot be found,
    /// or scraping encounters persistent issues.
    pub fn scrape_user_posts(&self, profile_url: &str, author_id: &str) -> Result<HashSet<String>> {
        let tab: Arc<Tab> = self.browser.new_tab()?;
        let tweet_selector = "article[data-testid='tweet']";

        // Attempt to load and set cookies from a previously saved file.
        let cookies_path = self.user_data_dir.join("manual_cookies.json");
        if cookies_path.exists() {
            println!("Found manual_cookies.json, attempting to load and set cookies from {:?}", cookies_path);
            match File::open(&cookies_path) {
                Ok(mut file) => {
                    let mut contents = String::new();
                    if file.read_to_string(&mut contents).is_ok() {
                        match serde_json::from_str::<Vec<headless_chrome::protocol::cdp::Network::Cookie>>(&contents) {
                            Ok(loaded_cookies_data) => {
                                let mut cookie_params_to_set: Vec<headless_chrome::protocol::cdp::Network::CookieParam> = Vec::new();
                                for cookie_data in &loaded_cookies_data {
                                    // Map loaded cookie data to CookieParam for setting.
                                    cookie_params_to_set.push(headless_chrome::protocol::cdp::Network::CookieParam {
                                        name: cookie_data.name.clone(),
                                        value: cookie_data.value.clone(),
                                        url: None, // URL is typically inferred from the current page or left as None for broad application.
                                        domain: Some(cookie_data.domain.clone()),
                                        path: Some(cookie_data.path.clone()),
                                        secure: Some(cookie_data.secure),
                                        http_only: Some(cookie_data.http_only),
                                        same_site: cookie_data.same_site.clone(),
                                        expires: if cookie_data.expires == -1.0 { None } else { Some(cookie_data.expires as headless_chrome::protocol::cdp::Network::TimeSinceEpoch) },
                                        priority: Some(cookie_data.priority.clone()),
                                        same_party: Some(cookie_data.same_party),
                                        source_scheme: Some(cookie_data.source_scheme.clone()),
                                        source_port: Some(cookie_data.source_port),
                                        partition_key: cookie_data.partition_key.clone(),
                                    });
                                }
                                if !cookie_params_to_set.is_empty() {
                                    let num_cookies_to_set = cookie_params_to_set.len(); 
                                    match tab.set_cookies(cookie_params_to_set) {
                                        Ok(_) => println!("Successfully set {} cookies from file.", num_cookies_to_set),
                                        Err(e) => println!("Error setting cookies from file: {}", e),
                                    }
                                } else {
                                    println!("No valid cookie data to set from file.");
                                }
                            }
                            Err(e) => println!("Error deserializing cookies from {:?}: {}", cookies_path, e),
                        }
                    } else {
                        println!("Error reading from cookie file {:?}", cookies_path);
                    }
                }
                Err(e) => println!("Error opening cookie file {:?}: {}", cookies_path, e),
            }
        } else {
            println!("No manual_cookies.json found at {:?}. Proceeding without setting manual cookies.", cookies_path);
        }

        println!("Navigating to profile page to check login status: {}", profile_url);
        tab.navigate_to(profile_url)?;
        tab.wait_until_navigated()?;
        println!("Page loaded. Waiting a bit for dynamic content...");
        thread::sleep(Duration::from_secs(5));

        // Determine if login is needed and attempt login using LoginHandler.
        let should_attempt_login_hardcoded = true; // Still hardcoded, but now passed to LoginHandler.
        let mut login_needed = true;

        let initial_login_trigger_sel = "a[data-testid='login']"; 
        if tab.find_element(initial_login_trigger_sel).is_err() {
            println!("Login button not found, potentially already logged in.");
            let success_element_sel_after_login = "a[data-testid='AppTabBar_Home_Link']";
            if tab.find_element(success_element_sel_after_login).is_ok() {
                println!("Verified logged in state via presence of '{}'.", success_element_sel_after_login);
                login_needed = false;
            } else {
                println!("Login button not found, but also couldn't confirm logged-in state via success element. Proceeding with login attempt if enabled.");
            }
        } else {
            println!("Login button found. Login is likely required.");
        }

        if login_needed && should_attempt_login_hardcoded {
            println!("Login required. Attempting login process...");
            // Define selectors for the login flow. These are specific to the target website.
            let username_sel_in_form = "input[name='text']"; 
            let next_button_sel_in_form = Some("div[class='css-175oi2r r-ywje51 r-nllxps r-jxj0sb r-1fkl15p r-16wqof'] button[class='css-175oi2r r-sdzlij r-1phboty r-rs99b7 r-lrvibr r-ywje51 r-184id4b r-13qz1uu r-2yi16 r-1qi8awa r-3pj75a r-1loqt21 r-o7ynqc r-6416eg r-1ny4l3l']");
            let password_sel_in_form = "input[name='password']"; 
            let final_login_button_sel_in_form = "div[data-testid='LoginForm_Footer_Container'] button"; 
            let success_url_part_after_login = Some("home"); 
            let success_element_sel_after_login = Some("a[data-testid='AppTabBar_Home_Link']");

            // Use the new LoginHandler for the login attempt
            LoginHandler::attempt_login(
                &tab, 
                initial_login_trigger_sel, 
                username_sel_in_form, 
                next_button_sel_in_form, 
                password_sel_in_form, 
                final_login_button_sel_in_form,
                success_url_part_after_login, 
                success_element_sel_after_login,
                &self.user_data_dir, // Pass user_data_dir for cookie saving
            )?;
            println!("Login process completed. Re-navigating to profile page to ensure correct state...");
            tab.navigate_to(profile_url)?;
            tab.wait_until_navigated()?;
            thread::sleep(Duration::from_secs(5));
        } else if !login_needed {
            println!("Skipping login attempt as session appears active.");
            if tab.get_url() != profile_url {
                println!("Not on profile URL, navigating to {}", profile_url);
                tab.navigate_to(profile_url)?;
                tab.wait_until_navigated()?;
                thread::sleep(Duration::from_secs(5));
            }
        } else {
            println!("Login is required but 'should_attempt_login_hardcoded' is false. Scraping may fail or be limited.");
        }

        let mut collected_html_set: HashSet<String> = HashSet::new();
        // Add author_id to the set for later processing/identification.
        collected_html_set.insert(format!("author_id: {}", author_id));

        let mut last_known_good_item_height: f64 = 250.0; // Initial estimate for tweet height
        let settle_time_after_scroll = Duration::from_secs(3);
        let settle_time_after_recovery_scroll = Duration::from_secs(5);
        const MAX_CONSECUTIVE_FAILED_CYCLES: usize = 3; 
        let mut consecutive_failed_cycles = 0;
        let mut first_scroll_performed = false;
        let mut total_elements_found_count: usize = 0;
        let mut reached_tweet_limit = false;
        const TWEET_LIMIT: usize = 50; // Max number of tweets to collect.


        println!("Starting iterative scrolling and HTML collection (limit {} tweets)...", TWEET_LIMIT);

        loop {
            if reached_tweet_limit {
                break;
            }

            let mut new_items_added_this_cycle = 0;
            let mut height_of_last_new_item_this_cycle = 0.0;

            println!("Scanning for tweet elements in current view to collect their HTML...");
            let elements_in_view_result = tab.find_elements(tweet_selector);
            
            let elements_in_view = match elements_in_view_result {
                Ok(elements) => elements,
                Err(e) => {
                    println!("Error finding tweet elements during scan: {}. Terminating.", e);
                    return Err(e.into()); // Return error if find_elements fails
                }
            };
            
            println!("Found {} potential tweet elements in view.", elements_in_view.len());
            total_elements_found_count += elements_in_view.len();

            // Iterate through found elements and collect their HTML.
            for el_arc in elements_in_view.iter() {
                if collected_html_set.len() >= TWEET_LIMIT + 1 { // +1 for the author_id string
                    println!("Reached the limit of {} tweet HTMLs. Stopping collection.", TWEET_LIMIT);
                    reached_tweet_limit = true;
                    break; 
                }
                match el_arc.get_content() {
                    Ok(html_content) => {
                        if collected_html_set.insert(html_content) {
                            new_items_added_this_cycle += 1;
                            if let Ok(model) = el_arc.get_box_model() {
                                if model.height > 0.0 {
                                    height_of_last_new_item_this_cycle = model.height;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("Warning: Failed to get HTML content for an element: {}", e);
                    }
                }
            }

            if reached_tweet_limit { 
                break;
            }

            // Adjust scrolling behavior based on whether new content was found.
            if new_items_added_this_cycle > 0 {
                println!("Successfully added HTML for {} new items this cycle.", new_items_added_this_cycle);
                consecutive_failed_cycles = 0; 
                if height_of_last_new_item_this_cycle > 10.0 {
                    last_known_good_item_height = height_of_last_new_item_this_cycle;
                    println!("Updated last_known_good_item_height to: {:.2}", last_known_good_item_height);
                }
            } else {
                println!("No new item HTML added in this cycle from the current view.");
                consecutive_failed_cycles += 1;
                println!("Consecutive failed cycles: {}/{}", consecutive_failed_cycles, MAX_CONSECUTIVE_FAILED_CYCLES);
                if consecutive_failed_cycles >= MAX_CONSECUTIVE_FAILED_CYCLES {
                    println!("Max consecutive failed cycles reached. Assuming end of content or no more new tweets visible.");
                    break;
                }
            }

            let scroll_amount;
            if !first_scroll_performed {
                // Initial larger scroll to get content loaded.
                scroll_amount = last_known_good_item_height * 1.5; 
                println!("Performing first scroll by: {:.2} (1.5 * {:.2})", scroll_amount, last_known_good_item_height);
                first_scroll_performed = true;
            } else if new_items_added_this_cycle > 0 { 
                // Scroll by roughly one item height if new items were found.
                scroll_amount = last_known_good_item_height * 1.0; 
                println!("Scrolling by last processed item height: {:.2}", scroll_amount);
            } else { 
                // If no new items, perform a larger "recovery" scroll.
                scroll_amount = (last_known_good_item_height * 2.5).max(800.0); 
                println!("Performing recovery scroll by: {:.2}", scroll_amount);
                thread::sleep(settle_time_after_recovery_scroll.saturating_sub(settle_time_after_scroll)); 
            }

            // Handle cases where scroll amount is too small, possibly indicating end of content.
            if scroll_amount <= 10.0 { 
                println!("Scroll amount {:.2} is too small. Considering this a failed cycle.", scroll_amount);
                consecutive_failed_cycles +=1; 
                println!("Incrementing failed cycles due to small scroll. Now {}/{}", consecutive_failed_cycles, MAX_CONSECUTIVE_FAILED_CYCLES);
                if consecutive_failed_cycles >= MAX_CONSECUTIVE_FAILED_CYCLES {
                    println!("Max consecutive failed cycles reached after small scroll. Assuming end of content.");
                    break;
                }
                if !reached_tweet_limit { 
                    // Attempt a final, larger recovery scroll if limit not yet reached.
                    let final_recovery_scroll = 1000.0; 
                    println!("Attempting final large recovery scroll by {:.2}", final_recovery_scroll);
                    tab.evaluate(&format!("window.scrollBy(0, {});", final_recovery_scroll), false)?;
                    thread::sleep(settle_time_after_recovery_scroll);
                    continue; 
                } else {
                    break; 
                }
            }

            // Execute the scroll.
            tab.evaluate(&format!("window.scrollBy(0, {});", scroll_amount), false)?;
            println!("Scrolled by {:.2}. Waiting for content to settle...", scroll_amount);
            thread::sleep(settle_time_after_scroll);
        }

        println!("\n--- Scraping Summary for author_id: {} ---", author_id);
        println!("Total tweet selector elements encountered (raw count): {}", total_elements_found_count);
        
        let actual_tweet_html_count = collected_html_set.len().saturating_sub(1); // Subtract 1 for the author_id string.

        println!("Finished scrolling and HTML collection. Target tweet limit: {}", TWEET_LIMIT);
        println!("Total unique tweet HTMLs collected: {}", actual_tweet_html_count);
        println!("Total items in HashSet (including author_id string): {}", collected_html_set.len());


        // Capture a screenshot if no actual tweet HTML content was collected.
        if actual_tweet_html_count == 0 {
            println!("No actual tweet HTML content was collected for {}.", author_id);
            if let Ok(data) = tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true) {
                let screenshot_path = format!("{}_no_html_collected_debug.png", author_id);
                if std::fs::write(&screenshot_path, data).is_ok() {
                    println!("No HTML collected. Screenshot saved to {}", screenshot_path);
                } else {
                    println!("No HTML collected. Failed to save debug screenshot.");
                }
            }
        }
        
        Ok(collected_html_set)
    }
}