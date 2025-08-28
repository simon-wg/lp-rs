use chrono::{Datelike, NaiveDate};

use crate::{
    errors::ScraperError,
    utils::{get_html, parse_year},
};

#[derive(Debug)]
pub struct StudyYear {
    pub year: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

pub fn get_study_year(url: &str) -> Result<StudyYear, ScraperError> {
    let html_body = get_html(url)
        .map_err(|e| ScraperError::ParseError(format!("Failed to fetch HTML: {}", e)))?;

    let year = parse_year(html_body.as_str()).unwrap_or(chrono::Utc::now().year());

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
