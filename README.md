# lp-rs

A Rust library for scraping academic study period and year information from Chalmers University websites.

## Features

- **Study Year Extraction**: Get the current academic year with start and end dates
- **Study Period Parsing**: Extract all study periods (typically 4 per academic year) with their date ranges
- **Error Handling**: Comprehensive error types for parsing and network issues
- **Type Safety**: Strongly typed data structures using Chrono for date handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lp-rs = "1.0.0"
```

## Usage

### Basic Example

```rust
use lp_rs::{get_study_periods, get_study_year};

fn main() {
    let url = "https://www.chalmers.se/utbildning/dina-studier/planera-och-genomfora-studier/datum-och-tider-for-lasaret/";

    // Get the current study year
    let year = get_study_year(url);
    match year {
        Ok(y) => println!("Current study year: {:#?}", y),
        Err(e) => eprintln!("Error fetching study year: {}", e),
    }

    // Get all study periods
    let periods = get_study_periods(url);
    match periods {
        Ok(ps) => {
            for period in ps {
                println!("{:#?}", period);
            }
        }
        Err(e) => eprintln!("Error fetching study periods: {}", e),
    }
}
```

### Data Structures

#### StudyYear

```rust
pub struct StudyYear {
    pub year: i32,           // Academic year (e.g., 2024)
    pub start_date: NaiveDate, // Start date (typically August 1st)
    pub end_date: NaiveDate,   // End date (typically July 31st)
}
```

#### StudyPeriod

```rust
pub struct StudyPeriod {
    pub year: i32,           // Academic year
    pub period: i32,         // Period number (1-4)
    pub start_date: NaiveDate, // Period start date
    pub end_date: NaiveDate,   // Period end date
}
```

### Error Handling

The library provides comprehensive error handling through the `ScraperError` enum:

```rust
pub enum ScraperError {
    ParseError(String),      // HTML parsing or data extraction errors
    DateParseError(String),  // Date parsing errors
}
```

## Dependencies

- `chrono`: Date and time handling
- `regex`: Regular expression parsing
- `reqwest`: HTTP client for fetching web content
- `scraper`: HTML parsing and CSS selector support

## Running Examples

```bash
cargo run --example basic_usage
```

## License

This project is licensed under the MIT License (MIT)
