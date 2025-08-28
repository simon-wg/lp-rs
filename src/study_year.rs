use chrono::NaiveDate;

use crate::{
    errors::ScraperError,
    utils::{get_html, parse_year},
};

/// Represents an academic year with its start and end dates.
#[derive(Debug)]
pub struct StudyYear {
    /// The academic year (e.g., 2024)
    pub year: i32,
    /// Start date of the academic year (typically August 1st)
    pub start_date: NaiveDate,
    /// End date of the academic year (typically July 31st)
    pub end_date: NaiveDate,
}

/// Scrapes the study year information from the given URL.
/// Returns a StudyYear struct with the academic year and its date range.
pub fn get_study_year(url: &str) -> Result<StudyYear, ScraperError> {
    let html_body = get_html(url)
        .map_err(|e| ScraperError::ParseError(format!("Failed to fetch HTML: {}", e)))?;

    let year = parse_year(html_body.as_str())?;

    Ok(StudyYear {
        year: year,
        start_date: NaiveDate::from_ymd_opt(year, 8, 1).ok_or_else(|| {
            ScraperError::DateParseError(format!("Invalid start date for year {}", year))
        })?,
        end_date: NaiveDate::from_ymd_opt(year + 1, 7, 31).ok_or_else(|| {
            ScraperError::DateParseError(format!("Invalid end date for year {}", year + 1))
        })?,
    })
}
