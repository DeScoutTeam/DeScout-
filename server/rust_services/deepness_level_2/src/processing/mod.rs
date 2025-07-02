//! This module contains components responsible for processing collected raw data,
//! transforming it into structured formats suitable for database interaction and analysis.

/// Provides functionality to process raw HTML data into structured data models.
pub mod html_processor;

/// Defines traits for data processing, enabling interchangeable processing strategies.
pub mod traits;

/// Re-exports the `HtmlSetProcessor` trait for easy access.
pub use traits::HtmlSetProcessor;