use chrono::{Datelike, Local};

use crate::Year;

pub fn current_year() -> Year {
    let local = Local::now();
    let date = local.date_naive();
    let (_, year) = date.year_ce();
    year
}
