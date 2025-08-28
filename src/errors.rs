use std::fmt;

/// Custom error type for scraper operations.
#[derive(Debug)]
pub enum ScraperError {
    /// Error occurred during HTML parsing or data extraction
    ParseError(String),
    /// Error occurred while parsing dates
    DateParseError(String),
}

impl fmt::Display for ScraperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        match self {
            ScraperError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ScraperError::DateParseError(msg) => write!(f, "Date parse error: {}", msg),
        }
    }
}

impl std::error::Error for ScraperError {}
