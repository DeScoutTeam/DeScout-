//! This module provides the `LoginHandler` for managing login
//! procedures on web platforms using `headless_chrome`.
//! It includes logic for filling forms, clicking buttons, and
//! handling session persistence via cookies.

use anyhow::{anyhow, Result};
use headless_chrome::{
    Tab,
    protocol::cdp::{Page, Network::{CookieParam, TimeSinceEpoch}},
};
use std::{
    fs::{File},
    io::{Write},
    path::PathBuf,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

/// `LoginHandler` provides static methods to handle login flows
/// and cookie management within a `headless_chrome` session.
pub struct LoginHandler;

impl LoginHandler {
    /// Attempts to log into a social media platform using provided selectors and credentials.
    ///
    /// This function simulates a user login flow by interacting with web elements
    /// identified by CSS selectors. It handles multi-step login processes
    /// (e.g., username, then password, potentially an intermediate verification step).
    /// It also attempts to load and save cookies to maintain sessions.
    ///
    /// # Arguments
    /// * `tab` - An `Arc<Tab>` representing the current browser tab.
    /// * `initial_login_trigger_selector` - CSS selector for the button that initiates the login flow.
    /// * `username_selector_in_form` - CSS selector for the username input field within the login form.
    /// * `next_button_selector_in_form` - Optional CSS selector for the "Next" button after entering the username.
    /// * `password_selector_in_form` - CSS selector for the password input field.
    /// * `final_login_button_selector_in_form` - CSS selector for the final login submission button.
    /// * `success_indicator_url_part` - An optional string part that should appear in the URL upon successful login.
    /// * `success_indicator_selector` - An optional CSS selector for an element that indicates successful login.
    /// * `user_data_dir` - The path to the user data directory where cookies should be saved.
    ///
    /// # Returns
    /// `Result<()>`: `Ok(())` if login appears successful, otherwise an `anyhow::Error`.
    ///
    /// # Errors
    /// Returns an error if elements are not found, interactions fail, or login
    /// cannot be confirmed within the timeout.
    pub fn attempt_login(
        tab: &Arc<Tab>,
        initial_login_trigger_selector: &str,
        username_selector_in_form: &str,
        next_button_selector_in_form: Option<&str>,
        password_selector_in_form: &str,
        final_login_button_selector_in_form: &str,
        success_indicator_url_part: Option<&str>,
        success_indicator_selector: Option<&str>,
        user_data_dir: &PathBuf, // Added to save cookies
    ) -> Result<()> {
        // Placeholder credentials - IMPORTANT: Replace with actual secure credentials
        let username = "@bot_username"; 
        let password = "bot_password"; 
        let email = "bot_temp_email"; 

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
                // If the trigger isn't found, check if we're already on a login form or logged in.
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

        // Inject script to bypass some common bot detection mechanisms.
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
        
        // Check for an intermediate phone/email verification step.
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
                                "   Name: {:?}, Value: {:?}, Domain: {:?}, Path: {:?}, Expires: {:?}, HttpOnly: {:?}, Secure: {:?}",
                                cookie_obj.name,
                                cookie_obj.value,
                                cookie_obj.domain,
                                cookie_obj.path,
                                cookie_obj.expires,
                                cookie_obj.http_only,
                                cookie_obj.secure
                            );
                        }
                        let cookies_path = user_data_dir.join("manual_cookies.json");
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
            // Capture screenshot on login failure for debugging.
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
}