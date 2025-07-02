//! The `tools` module provides utility components for scraping workflows.
//!
//! Currently, it contains the `login_handler` submodule, which implements
//! logic to handle login flows, including form filling, button clicking,
//! session persistence via cookies, and handling common web login challenges.

/// Provides login handling utilities for web scraping, including form interactions
/// and session management.
pub mod login_handler;