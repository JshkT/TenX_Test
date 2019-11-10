use std::fs::File;
use std::io::prelude::*;

use std::collections::LinkedList;
use crate::{datetime_helpers, PriceUpdate};


pub fn contents_processor(input_contents: &str) -> LinkedList<[&str; 6]> {
    let contlines = input_contents.lines();
    let mut cont_elem;
//    println!();

    let mut i = 0;
    let mut price_update_array: [&str; 6] = [" ";6];
    let mut list: LinkedList<[&str; 6]> = LinkedList::new();

    for line in contlines {
//        println!("{}", line);
        cont_elem = line.split_whitespace();


        for elem in cont_elem {
//            println!("{}", elem);
            price_update_array[i] = elem;
            i+=1;
        }
        list.push_back(price_update_array);
        i = 0;
    }
    return list;

}

pub fn get_contents_from_txt(input_path: &str) -> String {
    let mut file = File::open(input_path).expect("Cannot open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Cannot read the file.");
    return contents
}

pub fn get_node(input_array: [&str;6]) -> PriceUpdate {
    let output_price_update = PriceUpdate {
        timestamp: datetime_helpers::get_datetime(input_array[0]),
        exchange: input_array[1].to_string(),
        source_currency: input_array[2].to_string(),
        destination_currency: input_array[3].to_string(),
        forward_factor: input_array[4].parse().unwrap(),
        backward_factor: input_array[5].parse().unwrap()
    };
    return output_price_update;
}