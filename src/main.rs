extern crate chrono;

use chrono::{DateTime, FixedOffset};

mod datetime_helpers;
mod io_helpers;


pub struct PriceUpdate {
    timestamp: DateTime<FixedOffset>,
    exchange: String,
    source_currency: String,
    destination_currency: String,
    forward_factor: f32,
    backward_factor: f32
}

fn main() {
    println!("Begin");

    let input_path = "input.txt";
    let contents = io_helpers::get_contents_from_txt(input_path);

//    println!("File Contents:\n\n{}", contents);

    let mut list = io_helpers::contents_processor(&contents);

//    for n in list{
//        for m in &n{
//            println!("+++ {}", m);
//        }
//    }
    println!("{}", list.len());
    let test_array = list.pop_back().unwrap();
    println!("{}", test_array[0]);

    let node: PriceUpdate = io_helpers::get_node(test_array);

    println!("{}", node.exchange);

    while list.len() > 0 {
        list.pop_back();
    }


}