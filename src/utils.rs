use std::fmt::Display;

use chrono;
use chrono::{DateTime, NaiveDateTime, Utc};
use smartcore::numbers::basenum::Number;

pub fn seconds_to_date(epoch: u32) -> String {
    let naive = NaiveDateTime::from_timestamp_opt(epoch as i64, 0).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}


pub fn pad(a: &str, length: usize, filling: &str) -> String {
    format!("{}{}", a, filling.repeat(length as usize - a.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_test() {
        assert_eq!(seconds_to_date(1669899600), "2022-12-01 13:00:00.000000000");
    }
}
