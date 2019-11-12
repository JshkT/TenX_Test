extern crate chrono;
extern crate petgraph;


use chrono::{DateTime, FixedOffset};
use std::collections::{HashMap, LinkedList};
use std::io;
use std::io::BufRead;
use petgraph::Graph;

use crate::io_helpers::{is_request, price_update};
use crate::graph_helpers::{vertex_factory_array};
use petgraph::stable_graph::EdgeIndex;
use std::ops::Index;
use petgraph::prelude::NodeIndex;

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
    let mut vertices_vector: Vec<Vertex> = Vec::new();
    let mut edges_hashmap: HashMap<(Vertex, Vertex), (DateTime<FixedOffset>, f32)>;

    let mut graph = Graph::<Vertex, Edge>::new();
    let mut graph2 = Graph::<Vertex, Edge>::new();
    let mut vertex_index: Vec<Vertex> = Vec::new();
    let mut vertex_index3: Vec<NodeIndex> = Vec::new();
    let mut vertex_index2: Vec<Vertex> = Vec::new();
    let mut edge_index: Vec<Edge> = Vec::new();


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
//                    vertices_vector.push(v);
                    let v1 = v.clone();
                    graph2.add_node(v);
                    vertex_index2.push(v1);

                } else {
                    //Do nothing.
                }

            }
            //-----------------EDGES-----------------------------------------------
            let vertex_source = Vertex{ exchange: incoming_price_update.exchange.clone(), currency: incoming_price_update.source_currency.clone()};
            let vertex_destination = Vertex { exchange: incoming_price_update.exchange.clone(), currency:incoming_price_update.destination_currency.clone() };

            let edge_forward = Edge{rate: incoming_price_update.forward_factor, timestamp: incoming_price_update.timestamp};
            let edge_backward = Edge{rate: incoming_price_update.backward_factor, timestamp: incoming_price_update.timestamp};

            match vertex_index.contains(&vertex_source) {
                true => {}
                false => {
                    let v_source = graph.add_node(vertex_source.clone());
                    vertex_index.push(vertex_source.clone());
                    vertex_index3.push(v_source);
                    println!("VINDEX BUILT IN: {}",graph.index(v_source).currency);
                }
            }
            match vertex_index.contains(&vertex_destination) {
                true => {}
                false => {
                    let v_destination_index = graph.add_node(vertex_destination.clone());
                    vertex_index.push(vertex_destination.clone());
                    vertex_index3.push(v_destination_index);
                }
            }
            let source_ind = vertex_index.iter().position(|x| x.eq(&vertex_source));
            match source_ind {
                None => { println!("NOT FOUND") },
                Some(_0) => { println!("Found at index: {}", source_ind.unwrap()) }
            }

            let dest_ind = vertex_index.iter().position(|x| x.eq(&vertex_destination));
            match dest_ind {
                None =>{println!("NOT FOUND")},
                Some(_0) =>{println!("Found at index: {}",dest_ind.unwrap())}
            }

            //check if edge between source and dest exists
            let edge_index = graph.find_edge(vertex_index3[source_ind.unwrap()], vertex_index3[dest_ind.unwrap()]);
            match edge_index {
                None => {
                    // If not, simply add new edge.
                    let edge_index = graph.add_edge(vertex_index3[source_ind.unwrap()], vertex_index3[dest_ind.unwrap()], edge_forward);
                }
                Some(_0) => {
                    // if one exists, only update if the new rate is more recent.
                    if datetime_helpers::is_more_recent(incoming_price_update.timestamp,
                                                        graph.edge_weight(edge_index.unwrap()).unwrap().timestamp) {
                        let new_edge = Edge{ rate: incoming_price_update.forward_factor, timestamp: incoming_price_update.timestamp };
                        graph.update_edge(vertex_index3[source_ind.unwrap()], vertex_index3[dest_ind.unwrap()],new_edge);
                    } else {
                        // do nothing.
                    }


                }
            }

//            graph.contains_edge(, )

//            graph.index()
//            let res1 = edge_index.iter().position(|x| )
//            graph2.extend_with_edges(&[]);

        }


        println!("There are {} vertices. ", vertices_vector.len());
        let v = Vertex{exchange: "KRAKEN".to_string(), currency: "BTC".to_string()};
        println!("Vector Exists: {} ", vertices_vector.contains(&v));

        println!("There are {} vertices in graph. ", graph2.node_count());
        let v = Vertex{exchange: "KRAKEN".to_string(), currency: "BTC".to_string()};
        println!("Vector2 Exists: {} ", vertex_index2.contains(&v));
        println!("22222 {}", vertex_index2[0].currency.to_string());

        //Code to find index of some vertex.
        let res1 = vertex_index.iter().position(|x| x.eq(&v));
        match res1 {
            None =>{println!("NOT FOUND")},
            Some(_0) =>{println!("Found at index: {}",res1.unwrap())}
        }
        println!("{:?}", res1);
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

}

