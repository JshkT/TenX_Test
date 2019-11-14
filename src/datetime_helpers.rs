extern crate chrono;

use chrono::{DateTime, FixedOffset};

pub fn get_datetime(input_string: &str) -> DateTime<FixedOffset> {
    let dt = DateTime::parse_from_rfc3339(input_string);
    return dt.unwrap();
}

pub fn is_more_recent(dt_candidate: DateTime<FixedOffset>, dt_existing: DateTime<FixedOffset>) -> bool {
    if dt_candidate > dt_existing {
        println!("dt_candidate is more recent: {}", dt_candidate);
        return true
    } else {
        println!("dt_existing is more recent: {}", dt_existing);
        return false
    }
}