use chrono::{Datelike, NaiveDate};
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use std::fmt;

const URL: &str = "https://www.chalmers.se/utbildning/dina-studier/planera-och-genomfora-studier/datum-och-tider-for-lasaret/";

#[derive(Debug)]
pub struct StudyPeriod {
    pub year: i32,
    pub period: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug)]
pub struct StudyYear {
    pub year: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

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

fn parse_year(html_body: &str) -> Option<i32> {
    let study_year_re = Regex::new(r#"Läsår (?P<year>\d{4})/\d{4}"#).unwrap();

    study_year_re
        .captures(&html_body)?
        .name("year")?
        .as_str()
        .parse::<i32>()
        .ok()
}

fn get_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(url)?.text()?;
    Ok(response)
}
