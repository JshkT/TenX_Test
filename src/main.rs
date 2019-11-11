extern crate chrono;

use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use crate::io_helpers::is_request;

mod datetime_helpers;
mod io_helpers;
mod graph_helpers;


pub struct PriceUpdate {
    timestamp: DateTime<FixedOffset>,
    exchange: String,
    source_currency: String,
    destination_currency: String,
    forward_factor: f32,
    backward_factor: f32
}

pub struct Vertex {
    exchange: String,
    currency: String
}
pub struct Edge {
    source_vertex: Vertex,
    destination_vertex: Vertex,
    rate: f32
}

pub struct ExchangeRateRequest {
    source_exchange: String,
    source_currency: String,
    destination_exchange: String,
    destination_currency: String
}

fn main() {
    println!("Begin");
    let stdin = io::stdin();
    let mut is_request = false;

    for line in stdin.lock().lines(){
//        println!("stdin read: {}", line.unwrap());
        let request = &line.unwrap();
        is_request = io_helpers::is_request(String::from(request));

        if io_helpers::is_request(request.to_string()) {
            io_helpers::exchange_rate_request(request.split_whitespace())
        } else {
            io_helpers::price_update(request.split_whitespace())
        }


//        match &is_request {
//            true => io_helpers::exchange_rate_request(request.split_whitespace()),
//            false => io_helpers::price_update(request.split_whitespace()),
//        }
    }


    let input_path = "input.txt";
    let contents = io_helpers::get_contents_from_txt(input_path);

//    println!("File Contents:\n\n{}", contents);

    let mut list = io_helpers::contents_processor(&contents);


    println!("list len: {}", list.len());
    let test_array = list.pop_back().unwrap();
    println!("{}", test_array[0]);

    let node: PriceUpdate = io_helpers::get_node(test_array);

    println!("{}", node.exchange);

    let test_vertices = graph_helpers::vertex_factory(node);
    println!("{}", test_vertices[0].currency);


    while list.len() > 0 {
        list.pop_back();
    }

}

