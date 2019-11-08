extern crate chrono;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, FixedOffset, Utc};
use chrono::format::ParseError;

pub fn get_datetime(input_string: &str) -> DateTime<FixedOffset> {
    let dt = DateTime::parse_from_rfc3339(input_string);
    return dt.unwrap();
}