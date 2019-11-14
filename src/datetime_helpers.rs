/* Written by Joshua Tan in 2019
* For TenX Technical Exercise
* (The Exchange Rate Path Problem)
*/

extern crate chrono;

use chrono::{DateTime, FixedOffset};
use crate::DEBUG;

pub fn get_datetime(input_string: &str) -> DateTime<FixedOffset> {
    let dt = DateTime::parse_from_rfc3339(input_string);
    return dt.unwrap();
}

pub fn is_more_recent(dt_candidate: DateTime<FixedOffset>, dt_existing: DateTime<FixedOffset>) -> bool {
    if dt_candidate > dt_existing {
        if DEBUG {
            println!("dt_candidate is more recent: {}", dt_candidate);
        }
            return true
    } else {
        if DEBUG {
            println!("dt_existing is more recent: {}", dt_existing);
        }
        return false
    }
}