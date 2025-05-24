use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use headless_chrome::{
    Browser, LaunchOptions, Tab,
    browser::tab::element::Element,
    protocol::cdp::{Page, Network::{CookieParam, CookieSameSite, CookiePriority, CookieSourceScheme, TimeSinceEpoch}},
};
use serde::{Serialize, Deserialize};
use std::{
    collections::HashSet,
    ffi::OsStr,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

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

fn get_attribute_value(element: &Element, attr_name: &str) -> Result<Option<String>> {
    if let Some(attrs) = element.get_attributes()? {
        for pair in attrs.chunks(2) {
            if pair.len() == 2 && pair[0] == attr_name {
                return Ok(Some(pair[1].clone()));
            }
        }
    }
    Ok(None)
}

pub struct UserProfileScraper {
    browser: Browser,
    user_data_dir: PathBuf,
}

impl UserProfileScraper {
    pub fn new() -> Result<Self> {
        let run_headless_hardcoded = false;
        let user_data_dir_path = PathBuf::from("./chrome_scraper_profile");

        if !user_data_dir_path.exists() {
            fs::create_dir_all(&user_data_dir_path)?;
            println!("Created user data directory: {:?}", user_data_dir_path);
        } else {
            println!("Using existing user data directory: {:?}", user_data_dir_path);
        }

        let mut launch_options_builder = LaunchOptions::default_builder();
        launch_options_builder.headless(run_headless_hardcoded);
        
        launch_options_builder.user_data_dir(Some(user_data_dir_path.clone()));

        launch_options_builder.args(vec![OsStr::new("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")]);
        
        launch_options_builder.window_size(Some((1920, 1080)));

        launch_options_builder.args(vec![OsStr::new("--disable-blink-features=AutomationControlled")]);
        
        let options = launch_options_builder.build()?;
        let browser = Browser::new(options)?;
        Ok(Self { browser, user_data_dir: user_data_dir_path })
    }

    fn attempt_login(
        &self,
        tab: &Arc<Tab>,
        initial_login_trigger_selector: &str,
        username_selector_in_form: &str,
        next_button_selector_in_form: Option<&str>,
        password_selector_in_form: &str,
        final_login_button_selector_in_form: &str,
        success_indicator_url_part: Option<&str>,
        success_indicator_selector: Option<&str>
    ) -> Result<()> {
        let username = "YOUR_USERNAME_HERE"; 
        let password = "YOUR_PASSWORD_HERE"; 
        let email = "YOUR_EMAIL_HERE"

        if username == "YOUR_USERNAME_HERE" || password == "YOUR_PASSWORD_HERE" || email == "YOUR_EMAIL_HERE"{
            println!("WARNING: Using placeholder credentials in attempt_login. Please update them with real ones if login is intended.");
        }

        println!("Looking for initial login trigger button on current page: {}", initial_login_trigger_selector);
        match tab.wait_for_element_with_custom_timeout(initial_login_trigger_selector, Duration::from_secs(10)) {
            Ok(login_trigger_button) => {
                 println!("Clicking initial login trigger button...");
                login_trigger_button.click()?;
            }
            Err(_) => {
                if tab.find_element(username_selector_in_form).is_err() {
                    println!("Initial login trigger not found, and username field also not immediately visible. Potentially already logged in or on an unexpected page.");
                    if let Some(selector) = success_indicator_selector {
                        if tab.find_element(selector).is_ok() {
                            println!("Login success indicator found. Assuming already logged in.");
                            return Ok(());
                        }
                    }
                     if let Some(url_part) = success_indicator_url_part {
                        let current_url_direct: String = tab.get_url();
                        if current_url_direct.contains(url_part) && !current_url_direct.contains("login") && !current_url_direct.contains("checkpoint") {
                             println!("Login success indicated by URL. Assuming already logged in.");
                            return Ok(());
                        }
                    }
                }
            }
        }
       
        thread::sleep(Duration::from_secs(5));

        let script = r#"
            Object.defineProperty(navigator, 'webdriver', {get: () => undefined});
            Object.defineProperty(navigator, 'languages', {get: () => ['en-US', 'en']});
            Object.defineProperty(navigator, 'plugins', {get: () => [1, 2, 3]});
        "#;
        tab.evaluate(script, false)?;

        println!("Attempting to input username into the form/modal...");
        match tab.wait_for_element_with_custom_timeout(username_selector_in_form, Duration::from_secs(5)) {
            Ok(username_field) => {
                username_field.type_into(username)?;
                thread::sleep(Duration::from_millis(700));

                if let Some(next_selector) = next_button_selector_in_form {
                    println!("Clicking 'Next' button (after username) in form/modal...");
                    let next_button = tab.wait_for_element(next_selector)?;
                    next_button.click()?;
                    thread::sleep(Duration::from_secs(4));
                }
            }
            Err(_) => {
                println!("Username input field not found. Assuming username step is skipped (e.g. session active for this part).");
            }
        }
        
        let phone_email_verification_input_selector = "input[data-testid='ocfEnterTextTextInput']";
        let phone_email_verification_next_button_selector = "button[data-testid='ocfEnterTextNextButton']";

        thread::sleep(Duration::from_secs(3)); 

        if let Ok(verification_input_field) = tab.find_element(phone_email_verification_input_selector) {
            println!("Phone/Email verification step detected. Attempting to fill and proceed...");
            verification_input_field.type_into(email)?; 
            thread::sleep(Duration::from_millis(700));

            println!("Clicking 'Next' button on phone/email verification page...");
            let verification_next_button = tab.wait_for_element(phone_email_verification_next_button_selector)?;
            verification_next_button.click()?;
            thread::sleep(Duration::from_secs(5));
        } else {
            println!("Phone/Email verification step not detected, proceeding to password input if needed.");
        }

        println!("Attempting to input password into the form/modal...");
        match tab.wait_for_element_with_custom_timeout(password_selector_in_form, Duration::from_secs(5)) {
            Ok(password_field) => {
                password_field.type_into(password)?;
                thread::sleep(Duration::from_millis(700));

                println!("Clicking final 'Login' button in form/modal...");
                let login_button = tab.wait_for_element(final_login_button_selector_in_form)?;
                login_button.click()?;
            }
            Err(_) => {
                 println!("Password input field not found. Assuming login is already complete or a different flow occurred.");
                if let Some(selector) = success_indicator_selector {
                    if tab.find_element(selector).is_ok() {
                        println!("Login success indicator found after attempting password step. Assuming already logged in.");
                        return Ok(());
                    }
                }
                if let Some(url_part) = success_indicator_url_part {
                    let current_url_direct: String = tab.get_url();
                    if current_url_direct.contains(url_part) && !current_url_direct.contains("login") && !current_url_direct.contains("checkpoint") {
                        println!("Login success indicated by URL after attempting password step. Assuming already logged in.");
                        return Ok(());
                    }
                }
            }
        }

        println!("Waiting for login to complete (e.g., up to 15 seconds)...");
        let login_complete_time = Instant::now();
        let mut login_successful = false;
        while login_complete_time.elapsed() < Duration::from_secs(15) {
            if let Some(url_part) = success_indicator_url_part {
                let current_url_direct: String = tab.get_url();
                if current_url_direct.contains(url_part) && 
                   !current_url_direct.contains("login") && 
                   !current_url_direct.contains("checkpoint") {
                    println!("Login success indicated by URL change to: {}", current_url_direct);
                    login_successful = true;
                    break; 
                }
            }

            if let Some(selector) = success_indicator_selector {
                if tab.find_element(selector).is_ok() {
                    println!("Login success indicated by element: {}", selector);
                    login_successful = true;
                    break;
                }
            }
            thread::sleep(Duration::from_secs(1));
        }

        if login_successful {
            println!("Login appears to be successful.");

            println!("Attempting to retrieve and log cookies...");
            match tab.get_cookies() {
                Ok(retrieved_cookies) => {
                    if retrieved_cookies.is_empty() {
                        println!("No cookies found for the current page/session to log or save.");
                    } else {
                        println!("Found {} cookies:", retrieved_cookies.len());
                        for cookie_obj in &retrieved_cookies { 
                            println!(
                                "  Name: {:?}, Value: {:?}, Domain: {:?}, Path: {:?}, Expires: {:?}, HttpOnly: {:?}, Secure: {:?}",
                                cookie_obj.name,
                                cookie_obj.value,
                                cookie_obj.domain,
                                cookie_obj.path,
                                cookie_obj.expires,
                                cookie_obj.http_only,
                                cookie_obj.secure
                            );
                        }
                        let cookies_path = self.user_data_dir.join("manual_cookies.json");
                        println!("Attempting to save cookies to {:?}", cookies_path);
                        match serde_json::to_string_pretty(&retrieved_cookies) {
                            Ok(json_cookies) => {
                                match File::create(&cookies_path) {
                                    Ok(mut file) => {
                                        if let Err(e) = file.write_all(json_cookies.as_bytes()) {
                                            println!("Error writing cookies to file: {}", e);
                                        } else {
                                            println!("Successfully saved cookies to {:?}", cookies_path);
                                        }
                                    }
                                    Err(e) => println!("Error creating cookie file {:?}: {}", cookies_path, e),
                                }
                            }
                            Err(e) => println!("Error serializing cookies to JSON: {}", e),
                        }
                    }
                }
                Err(e) => {
                    println!("Error retrieving cookies: {}", e);
                }
            }
            
            thread::sleep(Duration::from_secs(3)); 
            Ok(())
        } else {
            if let Ok(data) = tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true) {
                if std::fs::write("login_failure.png", data).is_ok() {
                    println!("Login failure screenshot saved to login_failure.png");
                } else {
                    println!("Failed to save login_failure.png");
                }
            }
            Err(anyhow!("Login failed or could not be confirmed within timeout. Check for CAPTCHAs, 2FA, incorrect credentials/selectors, or the phone/email verification step. Current URL: {}", tab.get_url()))
        }
    }

    fn extract_and_process_single_tweet_data(
        &self,
        tweet_element: &Element,
        author_id: &str,
        processed_tweet_ids: &mut HashSet<String>,
    ) -> Result<Option<Airdrop>> {
        let permalink_anchor_selector = "a[href*='/status/'][role='link']";
        let tweet_id_opt = match tweet_element.find_element(permalink_anchor_selector) {
            Ok(link_element) => get_attribute_value(&link_element, "href")?
                .and_then(|href| href.split("/status/").nth(1).map(|s| s.split('/').next().unwrap_or(s).to_string())),
            Err(_) => match tweet_element.find_element("a[href*='/status/']") { // Fallback
                Ok(any_link_element) => get_attribute_value(&any_link_element, "href")?
                    .and_then(|href| href.split("/status/").nth(1).map(|s| s.split('/').next().unwrap_or(s).to_string())),
                Err(e) => {
                    // Log if even the fallback fails to find a link element
                    // println!("Debug: Could not find any status link in element. Error: {}", e);
                    None
                }
            }
        };

        let tweet_id = match tweet_id_opt {
            Some(id) if !id.is_empty() => id,
            _ => {
                // Try to get some outer HTML for context if ID extraction fails
                // let outer_html = tweet_element.get_content().unwrap_or_else(|_| "Outer HTML not available".to_string());
                // println!("Debug: Failed to extract valid tweet ID. Element HTML (approx): {:.100}", outer_html.chars().take(100).collect::<String>());
                return Ok(None);
            }
        };

        if processed_tweet_ids.contains(&tweet_id) {
            // println!("Debug: Tweet ID {} already processed, skipping.", tweet_id);
            return Ok(None); 
        }

        let text_selector = "div[data-testid='tweetText']";
        let text = match tweet_element.find_element(text_selector) {
            Ok(text_element) => text_element.get_inner_text().ok(),
            Err(_) => tweet_element.get_inner_text().ok().map(|full_text| { // Fallback
                full_text.lines().filter(|line| !line.trim().is_empty()).collect::<Vec<_>>().join("\n")
            }),
        };

        let time_selector = "time[datetime]";
        let created_at_var: Option<DateTime<Utc>> = tweet_element.find_element(time_selector).ok().and_then(|time_el| {
            get_attribute_value(&time_el, "datetime").ok().flatten().and_then(|datetime_str| {
                DateTime::parse_from_rfc3339(&datetime_str).map(|dt| dt.with_timezone(&Utc)).ok()
            })
        });

        let mentioned_users_var = text.as_ref().map(|txt| {
            txt.split_whitespace()
                .filter(|w| w.starts_with('@'))
                .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '@').to_string())
                .filter(|s| !s.is_empty() && s != "@")
                .collect()
        }).unwrap_or_default();

        let links_selector = "a[href]";
        let links_elements = match tweet_element.find_elements(links_selector) {
            Ok(els) => els,
            Err(e) => {
                println!("Warning: Failed to find link elements for tweet ID {}: {}", tweet_id, e);
                Vec::new() // Return an empty vec if links can't be found, instead of erroring out the whole tweet
            }
        };
        let links: Vec<String> = links_elements.into_iter().filter_map(|link_el| {
            get_attribute_value(&link_el, "href").ok().flatten()
                .filter(|href_val| !href_val.starts_with('#') && !href_val.starts_with("javascript:"))
        }).collect();

        let post = Airdrop {
            tweetId: tweet_id.clone(),
            text,
            authorId: Some(author_id.to_string()),
            createdAt: created_at_var,
            savedAt: Utc::now(),
            deepness: 2,
            keywords: vec![],
            tokenName: None,
            mentionedUsers: mentioned_users_var,
            links,
        };

        processed_tweet_ids.insert(tweet_id);
        // println!("Debug: Successfully extracted and processed tweet ID {}", post.tweetId);
        Ok(Some(post))
    }


    pub fn scrape_user_posts(&self, profile_url: &str, author_id: &str) -> Result<()> {
        let tab: Arc<Tab> = self.browser.new_tab()?;
        let tweet_selector = "article[data-testid='tweet']";

        let cookies_path = self.user_data_dir.join("manual_cookies.json");
        if cookies_path.exists() {
            println!("Found manual_cookies.json, attempting to load and set cookies from {:?}", cookies_path);
            match File::open(&cookies_path) {
                Ok(mut file) => {
                    let mut contents = String::new();
                    if file.read_to_string(&mut contents).is_ok() {
                        match serde_json::from_str::<Vec<headless_chrome::protocol::cdp::Network::Cookie>>(&contents) {
                            Ok(loaded_cookies_data) => {
                                let mut cookie_params_to_set: Vec<CookieParam> = Vec::new();
                                for cookie_data in &loaded_cookies_data {
                                    cookie_params_to_set.push(CookieParam {
                                        name: cookie_data.name.clone(),
                                        value: cookie_data.value.clone(),
                                        url: None, 
                                        domain: Some(cookie_data.domain.clone()),
                                        path: Some(cookie_data.path.clone()),
                                        secure: Some(cookie_data.secure),
                                        http_only: Some(cookie_data.http_only),
                                        same_site: cookie_data.same_site.clone(),
                                        expires: if cookie_data.expires == -1.0 { None } else { Some(cookie_data.expires as TimeSinceEpoch) },
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

        let should_attempt_login_hardcoded = true; 
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
            let username_sel_in_form = "input[name='text']"; 
            let next_button_sel_in_form = Some("div[class='css-175oi2r r-ywje51 r-nllxps r-jxj0sb r-1fkl15p r-16wqof'] button[class='css-175oi2r r-sdzlij r-1phboty r-rs99b7 r-lrvibr r-ywje51 r-184id4b r-13qz1uu r-2yi16 r-1qi8awa r-3pj75a r-1loqt21 r-o7ynqc r-6416eg r-1ny4l3l']");
            let password_sel_in_form = "input[name='password']"; 
            let final_login_button_sel_in_form = "div[data-testid='LoginForm_Footer_Container'] button"; 
            let success_url_part_after_login = Some("home"); 
            let success_element_sel_after_login = Some("a[data-testid='AppTabBar_Home_Link']");

            self.attempt_login(
                &tab, initial_login_trigger_sel, username_sel_in_form, 
                next_button_sel_in_form, password_sel_in_form, final_login_button_sel_in_form,
                success_url_part_after_login, success_element_sel_after_login
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

        let mut posts: Vec<Airdrop> = Vec::new();
        let mut processed_tweet_ids: HashSet<String> = HashSet::new();
        let mut last_known_good_tweet_height: f64 = 250.0;
        let settle_time_after_scroll = Duration::from_secs(3);
        let settle_time_after_recovery_scroll = Duration::from_secs(5);
        const MAX_CONSECUTIVE_FAILED_CYCLES: usize = 3; // Increased for more tolerance
        let mut consecutive_failed_cycles = 0;
        let mut first_scroll_performed = false;

        println!("Starting iterative scrolling and processing...");

        loop {
            let mut new_tweets_this_cycle = 0;
            let mut height_of_last_new_tweet_this_cycle = 0.0;

            println!("Scanning for tweets in current view...");
            let elements_in_view = match tab.find_elements(tweet_selector) {
                Ok(elements) => elements,
                Err(e) => {
                    println!("Error finding tweet elements during scan: {}. Terminating.", e);
                    break;
                }
            };
            println!("Found {} potential tweet elements in view.", elements_in_view.len());

            for el_arc in elements_in_view.iter() {
                if let Some(airdrop) = self.extract_and_process_single_tweet_data(&el_arc, author_id, &mut processed_tweet_ids)? {
                    // println!("Processed new tweet: ID {}", airdrop.tweetId); // Already logged in extract_and_process
                    posts.push(airdrop);
                    new_tweets_this_cycle += 1;
                    if let Ok(model) = el_arc.get_box_model() {
                        if model.height > 0.0 {
                            height_of_last_new_tweet_this_cycle = model.height;
                        }
                    }
                }
            }

            if new_tweets_this_cycle > 0 {
                println!("Successfully processed {} new tweets this cycle.", new_tweets_this_cycle);
                consecutive_failed_cycles = 0; 
                if height_of_last_new_tweet_this_cycle > 10.0 {
                    last_known_good_tweet_height = height_of_last_new_tweet_this_cycle;
                    println!("Updated last_known_good_tweet_height to: {:.2}", last_known_good_tweet_height);
                }
            } else {
                println!("No new tweets processed in this cycle from the current view.");
                consecutive_failed_cycles += 1;
                println!("Consecutive failed cycles: {}/{}", consecutive_failed_cycles, MAX_CONSECUTIVE_FAILED_CYCLES);
                if consecutive_failed_cycles >= MAX_CONSECUTIVE_FAILED_CYCLES {
                    println!("Max consecutive failed cycles reached. Assuming end of content.");
                    break;
                }
            }

            let scroll_amount;
            if !first_scroll_performed {
                scroll_amount = last_known_good_tweet_height * 1.5;
                println!("Performing first scroll by: {:.2} (1.5 * {:.2})", scroll_amount, last_known_good_tweet_height);
                first_scroll_performed = true;
            } else if new_tweets_this_cycle > 0 { 
                scroll_amount = last_known_good_tweet_height * 1.0;
                println!("Scrolling by last processed tweet height: {:.2}", scroll_amount);
            } else { 
                // More aggressive recovery scroll if no new tweets but not yet at max failed cycles
                scroll_amount = (last_known_good_tweet_height * 2.5).max(800.0); 
                println!("Performing recovery scroll by: {:.2}", scroll_amount);
                // Apply longer settle time for recovery scrolls
                thread::sleep(settle_time_after_recovery_scroll.saturating_sub(settle_time_after_scroll)); 
            }

            if scroll_amount <= 10.0 { // If scroll amount is still too small even after recovery logic
                println!("Scroll amount {:.2} is too small. Breaking to avoid potential infinite loop.", scroll_amount);
                 if consecutive_failed_cycles < MAX_CONSECUTIVE_FAILED_CYCLES {
                    consecutive_failed_cycles +=1; // Count this as a failure if we didn't already hit max
                     println!("Incrementing failed cycles due to small scroll. Now {}/{}", consecutive_failed_cycles, MAX_CONSECUTIVE_FAILED_CYCLES);
                     if consecutive_failed_cycles >= MAX_CONSECUTIVE_FAILED_CYCLES {
                         println!("Max consecutive failed cycles reached after small scroll. Assuming end of content.");
                         break;
                     }
                     // Attempt one last large, fixed recovery scroll
                     let final_recovery_scroll = 1000.0; // A larger fixed amount
                     println!("Attempting final large recovery scroll by {:.2}", final_recovery_scroll);
                     tab.evaluate(&format!("window.scrollBy(0, {});", final_recovery_scroll), false)?;
                     thread::sleep(settle_time_after_recovery_scroll);
                     continue; // Re-enter loop to scan after this final recovery
                } else {
                    break; // Already at max fails, so break.
                }
            }

            tab.evaluate(&format!("window.scrollBy(0, {});", scroll_amount), false)?;
            println!("Scrolled by {:.2}. Waiting for content to settle...", scroll_amount);
            thread::sleep(settle_time_after_scroll);
        }

        println!("Finished scrolling and processing. Total posts collected: {}", posts.len());

        if posts.is_empty() {
            println!("No posts were ultimately saved.");
            if let Ok(data) = tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true) {
                let screenshot_path = format!("{}_no_posts_debug.png", author_id);
                if std::fs::write(&screenshot_path, data).is_ok() {
                    println!("No posts found. Screenshot saved to {}", screenshot_path);
                } else {
                    println!("No posts found. Failed to save debug screenshot.");
                }
            }
        } else {
             println!("Collected {} posts in total for {}.", posts.len(), author_id);
        }

        let file_path = format!("{}_posts_dynamic_scroll.json", author_id);
        let mut file = File::create(&file_path)?;
        let json = serde_json::to_string_pretty(&posts)?;
        file.write_all(json.as_bytes())?;

        println!("Successfully saved {} posts to {}", posts.len(), file_path);
        Ok(())
    }
}
