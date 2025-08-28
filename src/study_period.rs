use chrono::{Datelike, NaiveDate};
use regex::Regex;
use scraper::{Html, Selector};

use crate::{
    errors::ScraperError,
    utils::{get_html, parse_year},
};

#[derive(Debug)]
pub struct StudyPeriod {
    pub year: i32,
    pub period: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

pub fn get_study_periods(url: &str) -> Result<Vec<StudyPeriod>, ScraperError> {
    let html_body = get_html(url)
        .map_err(|e| ScraperError::ParseError(format!("Failed to fetch HTML: {}", e)))?;

    let html = Html::parse_document(html_body.as_str());
    let tr_selector = Selector::parse("tr")
        .map_err(|e| ScraperError::SelectorError(format!("Failed to parse selector: {}", e)))?;

    let current_year = parse_year(html_body.as_str()).unwrap_or(chrono::Utc::now().year());

    html.select(&tr_selector)
        .filter(|tr| tr.text().collect::<String>().contains("Läsperiod"))
        .map(|tr| parse_study_period(tr, current_year))
        .collect()
}

fn parse_study_period(tr: scraper::ElementRef, year: i32) -> Result<StudyPeriod, ScraperError> {
    let date_re = Regex::new(r#"^\d{4}-\d{2}-\d{2}$"#).unwrap();
    let study_period_re = Regex::new(r#"Läsperiod (\d)"#).unwrap();

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
        .filter(|s| date_re.is_match(s) || study_period_re.is_match(s))
        .cloned()
        .collect();

    if filtered_text.len() != 3 {
        return Err(ScraperError::ParseError(format!(
            "Expected 3 elements in filtered text, got {}: {:?}",
            filtered_text.len(),
            filtered_text
        )));
    }

    let captures = study_period_re
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
