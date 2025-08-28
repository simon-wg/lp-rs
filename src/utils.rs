use regex::Regex;

pub(crate) fn parse_year(html_body: &str) -> Option<i32> {
    let study_year_re = Regex::new(r#"Läsår (?P<year>\d{4})/\d{4}"#).unwrap();

    study_year_re
        .captures(&html_body)?
        .name("year")?
        .as_str()
        .parse::<i32>()
        .ok()
}

pub(crate) fn get_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(url)?.text()?;
    Ok(response)
}
