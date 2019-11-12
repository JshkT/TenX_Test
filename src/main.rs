extern crate chrono;

use chrono::{DateTime, FixedOffset};
use std::collections::{HashMap, LinkedList};
use std::io;
use std::io::BufRead;
use crate::io_helpers::is_request;
use crate::graph_helpers::*;

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

#[derive(Clone, Eq, PartialEq)]
pub struct Vertex {
    exchange: String,
    currency: String
}

#[derive(Clone, PartialEq)]
pub struct Edge {
    source_vertex: Vertex,
    destination_vertex: Vertex,
    rate: f32,
    timestamp: DateTime<FixedOffset>
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
    let mut vertices_list: LinkedList<Vertex> = LinkedList::new();
    let mut vertices_vector: Vec<Vertex> = Vec::new();
    let mut edges_hashmap: HashMap<(Vertex, Vertex), (DateTime<FixedOffset>, f32)>;
    let mut rate_graph: graph_helpers::Graph;


    for line in stdin.lock().lines(){
//        println!("stdin read: {}", line.unwrap());
        let request = &line.unwrap();
        is_request = io_helpers::is_request(String::from(request));

        if io_helpers::is_request(request.to_string()) {
            io_helpers::exchange_rate_request(request.split_whitespace());
            //------------Get best path-----------------------------------------
        } else {
            let incoming_price_update = io_helpers::price_update(request.split_whitespace());
            //-------------Update vertices and edges ----------------------------
            //-------------VERTICES----------------------------------------------
            // consider changing from linked list to array output from vertex_factory.
            let new_vertices_array = vertex_factory_array(&incoming_price_update);
            let mut vert_vec: Vec<Vertex> = Vec::new();

            for i_vertex in &new_vertices_array{
                let mut match_found = false;
                for j_vertex in &vertices_vector{
                    if i_vertex.eq(j_vertex){
                        match_found = true;
                    }
                }
                if match_found == false {
                    let v = Vertex{exchange: i_vertex.exchange.clone(), currency: i_vertex.currency.clone()};
                    vertices_vector.push(v);
                } else {
                    //Do nothing.
                }

            }
            //-----------------EDGES-----------------------------------------------

//            edges_hashmap.insert((new_vertices_array[0],new_vertices_array[1]), (incoming_price_update.timestamp, incoming_price_update.forward_factor));
//            edges_hashmap.insert((new_vertices_array[0],new_vertices_array[1]), (incoming_price_update.timestamp, incoming_price_update.backward_factor));


        }
        println!("There are {} vertices. ", vertices_list.len());
        let v = Vertex{exchange: "KRAKEN".to_string(), currency: "BTC".to_string()};
        println!("Vector Exists: {} ", vertices_vector.contains(&v));


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


    while list.len() > 0 {
        list.pop_back();
    }

}

