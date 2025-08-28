use std::fmt;

#[derive(Debug)]
pub enum ScraperError {
    ParseError(String),
    SelectorError(String),
    DateParseError(String),
}

impl fmt::Display for ScraperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        match self {
            ScraperError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ScraperError::SelectorError(msg) => write!(f, "Selector error: {}", msg),
            ScraperError::DateParseError(msg) => write!(f, "Date parse error: {}", msg),
        }
    }
}

impl std::error::Error for ScraperError {}
