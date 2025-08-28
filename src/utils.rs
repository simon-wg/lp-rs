use chrono::Datelike;
use regex::Regex;
use std::sync::LazyLock;

static STUDY_YEAR_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"Läsår (?P<year>\d{4})/\d{4}"#).unwrap());

pub(crate) fn parse_year(html_body: &str) -> Result<i32, crate::ScraperError> {
    if let Some(captures) = STUDY_YEAR_REGEX.captures(html_body) {
        if let Some(year_match) = captures.name("year") {
            return year_match.as_str().parse::<i32>().map_err(|e| {
                crate::ScraperError::ParseError(format!("Failed to parse year: {}", e))
            });
        }
    }

    Ok(chrono::Utc::now().year())
}

pub(crate) fn get_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(url)?.text()?;
    Ok(response)
}
