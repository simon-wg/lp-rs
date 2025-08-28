use lp_rs::{get_study_periods, get_study_year};

fn main() {
    let url = "https://www.chalmers.se/utbildning/dina-studier/planera-och-genomfora-studier/datum-och-tider-for-lasaret/";
    let year = get_study_year(url);
    match year {
        Ok(y) => println!("Current study year: {:#?}", y),
        Err(e) => eprintln!("Error fetching study year: {}", e),
    }
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
