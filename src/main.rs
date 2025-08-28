use chrono::{Datelike, NaiveDate};
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};

const URL: &str = "https://www.chalmers.se/utbildning/dina-studier/planera-och-genomfora-studier/datum-och-tider-for-lasaret/";

#[derive(Debug)]
struct StudyPeriod {
    year: i32,
    period: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

#[derive(Debug)]
struct StudyYear {
    year: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

fn main() {
    let html_body = get_html(URL).unwrap();
    let study_periods: Vec<StudyPeriod> = get_study_periods(&html_body);
    println!("{:#?}", study_periods);
}

fn get_study_periods(html_body: &str) -> Vec<StudyPeriod> {
    let html = Html::parse_document(html_body);
    let tr_selector = Selector::parse("tr").unwrap();

    let current_year = parse_year(html_body).unwrap_or(chrono::Utc::now().year());

    let mut study_periods: Vec<StudyPeriod> = Vec::new();
    for tr in html
        .select(&tr_selector)
        .filter(|tr| tr.text().collect::<String>().contains("Läsperiod"))
    {
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
        let date_re = Regex::new(r#"^\d{4}-\d{2}-\d{2}$"#).unwrap();
        let study_period_re = Regex::new(r#"Läsperiod (\d)"#).unwrap();
        let filtered_text: Vec<&str> = cleaned_text
            .iter()
            .filter(|s| date_re.is_match(s) || study_period_re.is_match(s))
            .cloned()
            .collect();
        assert!(filtered_text.len() == 3);
        let captures = study_period_re.captures(cleaned_text[0]).unwrap();
        assert!(captures.len() == 2);
        let period: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        study_periods.push(StudyPeriod {
            year: current_year,
            period: period,
            start_date: NaiveDate::parse_from_str(filtered_text[1], "%Y-%m-%d").unwrap(),
            end_date: NaiveDate::parse_from_str(filtered_text[2], "%Y-%m-%d").unwrap(),
        });
    }
    study_periods
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
