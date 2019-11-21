/* Written by Joshua Tan in 2019
* For TenX Technical Exercise
* (The Exchange Rate Path Problem)
*/

use crate::{datetime_helpers, ExchangeRateRequest, PriceUpdate};
use petgraph::graph::node_index;
use petgraph::Graph;
use std::str::SplitWhitespace;

pub fn exchange_rate_request(input: SplitWhitespace) -> ExchangeRateRequest {
    //    println!("GET EXCHANGE RATE FOR REQUEST:");
    let mut request_array: [&str; 5] = [" "; 5];
    let mut i = 0;
    for elem in input {
        request_array[i] = elem;
        i += 1;
    }
    let request = ExchangeRateRequest {
        source_exchange: request_array[1].to_string(),
        source_currency: request_array[2].to_string(),
        destination_exchange: request_array[3].to_string(),
        destination_currency: request_array[4].to_string(),
    };
    return request;
}

pub fn price_update(input: SplitWhitespace) -> PriceUpdate {
    //    println!("Incoming price update:");
    let mut price_update_array: [&str; 6] = [" "; 6];
    let mut i = 0;

    for elem in input {
        price_update_array[i] = elem;
        i += 1;
    }
    if i != 6 {
        println!("Error in input. Incorrect number of parameters.");
        //error
    }
    let pupdate = get_node(price_update_array);
    return pupdate;
}

pub fn get_node(input_array: [&str; 6]) -> PriceUpdate {
    let output_price_update = PriceUpdate {
        timestamp: datetime_helpers::get_datetime(input_array[0]),
        exchange: input_array[1].to_string(),
        source_currency: input_array[2].to_string(),
        destination_currency: input_array[3].to_string(),
        forward_factor: input_array[4].parse().unwrap(),
        backward_factor: input_array[5].parse().unwrap(),
    };
    return output_price_update;
}

pub fn print_results_part_one(rate_request: &ExchangeRateRequest, best_rate: &f32) {
    println!(
        "BEST_RATES_BEGIN <{}> <{}> <{}> <{}> <{:?}> ",
        rate_request.source_exchange,
        rate_request.source_currency,
        rate_request.destination_exchange,
        rate_request.destination_currency,
        best_rate
    );
}
pub fn print_results_part_two(path: &Option<Vec<usize>>, graph: &Graph<String, f32>) {
    match path {
        Some(v) => {
            for x in v {
                let node_name = graph.node_weight(node_index(*x));
                node_name.map(|node_name| println!("<{}>", node_name));
            }
        }
        None => println!("There is no path from source to desired destination"),
    }
}
