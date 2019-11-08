extern crate chrono;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, FixedOffset, Utc, TimeZone};
use chrono::format::ParseError;

use std::fs::File;
use std::io::prelude::*;

use std::str::{FromStr, SplitWhitespace};
use std::fmt::Display;

use std::io;
use std::any::Any;

use std::collections;
use std::ptr::null_mut;
use std::collections::LinkedList;

mod datetime_helpers;


struct PriceUpdate {
timestamp: DateTime<FixedOffset>,
exchange: String,
source_currency: String,
destination_currency: String,
forward_factor: f32,
backward_factor: f32
}

struct PriceUpdateString {
    timestamp: String,
    exchange: String,
    source_currency: String,
    destination_currency: String,
    forward_factor: String,
    backward_factor: String
}

fn main() {
    println!("Begin");
    let mut file = File::open("input.txt").expect("Cannot open file.");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Cannot read the file.");

//    println!("File Contents:\n\n{}", contents);

    let test_string = "2017-11-01T09:42:23+00:00";
    let dt = datetime_helpers::get_datetime(test_string);
//    println!("{}",dt.to_string());


    let mut contlines = contents.lines();
    let mut cont_elem;
    println!();

    let mut i = 0;
    let mut j = 0;
    let mut price_update_array: [&str; 6] = [" ";6];
    let mut list: LinkedList<[&str; 6]> = LinkedList::new();

    for line in contlines {
        println!("{}", line);
        cont_elem = line.split_whitespace();


        for elem in cont_elem {
            println!("{}", elem);
            price_update_array[i] = elem;
            i+=1;
        }
        list.push_back(price_update_array);
        i = 0;
        j += 1;
    }

    for n in list{
        for m in &n{
            println!("+++ {}", m);
        }
    }



    let test_price_update = PriceUpdate {
        timestamp: dt,
        exchange: "KRAKEN".to_string(),
        source_currency: "BTC".to_string(),
        destination_currency: "USD".to_string(),
        forward_factor: 1000.0,
        backward_factor: 0.0009
    };


}