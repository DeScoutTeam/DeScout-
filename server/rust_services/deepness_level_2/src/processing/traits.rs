//! This module defines traits for processing collections of HTML strings.

use crate::scraper::user_profile::Airdrop;
use anyhow::Result;
use std::collections::HashSet;

/// `HtmlSetProcessor` is a trait for types that can process a `HashSet` of HTML strings.
///
/// Implementors of this trait are expected to transform the raw HTML data
/// into a structured vector of `Airdrop` objects. This allows for different
/// strategies or versions of HTML processing to be used interchangeably.
pub trait HtmlSetProcessor {
    /// Processes a given `HashSet` of HTML strings into a `Vec` of `Airdrop` structs.
    ///
    /// # Arguments
    /// * `html_set` - A reference to a `HashSet` containing HTML strings to be processed.
    ///
    /// # Returns
    /// `Result<Vec<Airdrop>>`: A `Result` containing a vector of `Airdrop` structs if successful,
    /// or an `anyhow::Error` if processing fails.
    fn process_html_set(&self, html_set: &HashSet<String>) -> Result<Vec<Airdrop>>;
}