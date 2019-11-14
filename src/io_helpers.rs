use std::fs::File;
use std::io::prelude::*;

use std::collections::LinkedList;
use crate::{datetime_helpers, PriceUpdate, ExchangeRateRequest, DEBUG};
use std::str::SplitWhitespace;

pub fn is_request(line_input: String) -> bool {
//    println!("Starting process_line on: {}", line_input);

    if line_input.contains("EXCHANGE_RATE_REQUEST"){
//        println!("EXCHANGE_RATE_REQUEST");
//        exchange_rate_request(line_input.split_whitespace());
        return true;
    } else {
//        price_update(line_input.split_whitespace());
        return false;
    }
}

pub fn exchange_rate_request(input: SplitWhitespace) -> ExchangeRateRequest{
//    println!("GET EXCHANGE RATE FOR REQUEST:");
    let mut request_array: [&str; 5] = [" "; 5];
    let mut i=0;
    for elem in input {
        request_array[i] = elem;
        i+=1;
    }
    let request = ExchangeRateRequest{
        source_exchange: request_array[1].to_string(),
        source_currency: request_array[2].to_string(),
        destination_exchange: request_array[3].to_string(),
        destination_currency: request_array[4].to_string()
    };
    return request
}

pub fn price_update(input: SplitWhitespace) -> PriceUpdate {
//    println!("Incoming price update:");
    let mut price_update_array: [&str; 6] = [" "; 6];
    let mut i = 0;

    for elem in input {
        price_update_array[i] = elem;
        i+=1;
    }
    if i != 6 {
        println!("Error in input. Incorrect number of parameters.");
        //error
    }
    let pupdate = get_node(price_update_array);
    return pupdate

}

pub fn contents_processor(input_contents: &str) -> LinkedList<[&str; 6]> {
    let contlines = input_contents.lines();
    let mut cont_elem;


    let mut i = 0;
    let mut j = 0;
    let mut price_update_array: [&str; 6] = [" "; 6];
    let mut request_array: [&str; 4] = [" "; 4];
    let mut list: LinkedList<[&str; 6]> = LinkedList::new();

    for line in contlines {
//        println!("{}", line);
        cont_elem = line.split_whitespace();
        let mut is_request = false;
        for elem in cont_elem {
//            println!("{}", elem);
            if is_request == true {
                request_array[j] = elem;
                j+=1;
                if j >= 4 {
                    let request = ExchangeRateRequest{
                        source_exchange: request_array[0].to_string(),
                        source_currency: request_array[1].to_string(),
                        destination_exchange: request_array[2].to_string(),
                        destination_currency: request_array[3].to_string()
                    };
                    if DEBUG {
                        println!("ExchangeRateRequest: {}, {}, {}, {}",
                                 request.source_exchange,
                                 request.source_currency,
                                 request.destination_exchange,
                                 request.destination_currency
                        );
                    }

                    return list;
                }
            }
            else {
                is_request = false;
                if elem.eq("EXCHANGE_RATE_REQUEST") {
                println!("EXCHANGE_RATE_REQUEST");
                is_request = true;
                // Call function for exchange rate with data currently available.
            }
                price_update_array[i] = elem;
                i+=1;
            }

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