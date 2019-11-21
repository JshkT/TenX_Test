/* Written by Joshua Tan in 2019
* For TenX Technical Exercise
* (The Exchange Rate Path Problem)
*/

extern crate chrono;

use chrono::{DateTime, FixedOffset};

/*
* Takes a string and returns it in the form DateTime<FixedOffset>
*/
pub fn get_datetime_from_string(input_string: &str) -> DateTime<FixedOffset> {
    let dt = DateTime::parse_from_rfc3339(input_string);
    return dt.unwrap();
}

/*
* Compares two DateTimes and returns true if the candidate is more recent than the existing.
*/
pub fn is_more_recent(
    dt_candidate: DateTime<FixedOffset>,
    dt_existing: DateTime<FixedOffset>,
) -> bool {
    let diff = dt_candidate - dt_existing;
    //    let diff = dt_existing - dt_candidate;
    let diff_nano = chrono::Duration::num_nanoseconds(&diff);

    match diff_nano {
        Some(t) => {
            if t > 0 {
                return true;
            } else {
                return false;
            }
        }
        None => {
            return false;
        }
    }
}
