use chrono::{Datelike, Local};

pub fn get_date_string(year: Option<i32>, month: Option<u32>, day: Option<u32>) -> String {
    let now = Local::now();
    let date_string = format!(
        "{}-{}-{}",
        match year {
            Some(year) => year,
            None => now.year(),
        },
        match month {
            Some(month) => month,
            None => now.month(),
        },
        match day {
            Some(day) => day,
            None => now.day(),
        }
    );
    date_string
}
