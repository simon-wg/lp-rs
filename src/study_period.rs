use chrono::NaiveDate;
use regex::Regex;
use scraper::{Html, Selector};
use std::sync::LazyLock;

use crate::{
    errors::ScraperError,
    utils::{get_html, parse_year},
};

static DATE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\d{4}-\d{2}-\d{2}$"#).unwrap());
static STUDY_PERIOD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"Läsperiod (\d)"#).unwrap());

/// Represents a study period with its year, period number, and date range.
#[derive(Debug)]
pub struct StudyPeriod {
    /// The academic year (e.g., 2024)
    pub year: i32,
    /// The period number within the year (1 to 4)
    pub period: i32,
    /// Start date of the study period
    pub start_date: NaiveDate,
    /// End date of the study period
    pub end_date: NaiveDate,
}

/// Scrapes study periods from the given URL.
/// Returns a vector of StudyPeriod structs containing the parsed data.
pub fn get_study_periods(url: &str) -> Result<Vec<StudyPeriod>, ScraperError> {
    let html_body = get_html(url)
        .map_err(|e| ScraperError::ParseError(format!("Failed to fetch HTML: {}", e)))?;

    let html = Html::parse_document(html_body.as_str());
    let tr_selector = Selector::parse("tr").unwrap();

    let current_year = parse_year(html_body.as_str())?;

    html.select(&tr_selector)
        .filter(|tr| tr.text().collect::<String>().contains("Läsperiod"))
        .map(|tr| parse_study_period(tr, current_year))
        .collect()
}

fn parse_study_period(tr: scraper::ElementRef, year: i32) -> Result<StudyPeriod, ScraperError> {
    let cleaned_text: Vec<&str> = tr
        .text()
        .filter_map(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
        .collect();

    let filtered_text: Vec<&str> = cleaned_text
        .iter()
        .filter(|s| DATE_REGEX.is_match(s) || STUDY_PERIOD_REGEX.is_match(s))
        .cloned()
        .collect();

    if filtered_text.len() != 3 {
        return Err(ScraperError::ParseError(format!(
            "Expected 3 elements in filtered text, got {}: {:?}",
            filtered_text.len(),
            filtered_text
        )));
    }

    let captures = STUDY_PERIOD_REGEX
        .captures(cleaned_text[0])
        .ok_or_else(|| ScraperError::ParseError("Failed to capture study period".to_string()))?;

    if captures.len() != 2 {
        return Err(ScraperError::ParseError(format!(
            "Expected 2 captures, got {}",
            captures.len()
        )));
    }

    let period_str = captures
        .get(1)
        .ok_or_else(|| ScraperError::ParseError("Failed to get period capture group".to_string()))?
        .as_str();

    let period: i32 = period_str.parse().map_err(|e| {
        ScraperError::ParseError(format!("Failed to parse period '{}': {}", period_str, e))
    })?;

    let start_date = NaiveDate::parse_from_str(filtered_text[1], "%Y-%m-%d").map_err(|e| {
        ScraperError::DateParseError(format!(
            "Failed to parse start date '{}': {}",
            filtered_text[1], e
        ))
    })?;

    let end_date = NaiveDate::parse_from_str(filtered_text[2], "%Y-%m-%d").map_err(|e| {
        ScraperError::DateParseError(format!(
            "Failed to parse end date '{}': {}",
            filtered_text[2], e
        ))
    })?;

    Ok(StudyPeriod {
        year,
        period,
        start_date,
        end_date,
    })
}
