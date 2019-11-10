extern crate chrono;

use chrono::{DateTime, FixedOffset};

pub fn get_datetime(input_string: &str) -> DateTime<FixedOffset> {
    let dt = DateTime::parse_from_rfc3339(input_string);
    return dt.unwrap();
}