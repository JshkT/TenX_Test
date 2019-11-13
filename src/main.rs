extern crate chrono;
extern crate petgraph;
extern crate floyd_warshall;
#[macro_use]
extern crate matrix;

use chrono::{DateTime, FixedOffset};

use std::io;
use std::io::{BufRead, Write};
use petgraph::Graph;
use petgraph::algo;
use matrix::prelude::*;

use crate::io_helpers::{is_request, price_update};
use crate::graph_helpers::{vertex_factory_array, get_index_from_vertex};
use petgraph::stable_graph::EdgeIndex;
use std::ops::Index;
use petgraph::prelude::NodeIndex;
use petgraph::graph::node_index;
use matrix::prelude::{Compressed, Conventional};
use matrix::format::compressed::Variant;

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

    let mut graph = Graph::<String, f32>::new();
    let mut vertex_data: Vec<Vertex> = Vec::new();
    let mut vertex_index: Vec<NodeIndex> = Vec::new();

    let mut edge_index: Vec<EdgeIndex> = Vec::new();
    let mut edge_data: Vec<Edge> = Vec::new();

    for line in stdin.lock().lines(){
        let request = &line.unwrap();
        is_request = io_helpers::is_request(String::from(request));

        if io_helpers::is_request(request.to_string()) {
            io_helpers::exchange_rate_request(request.split_whitespace());
            //------------Get best path-----------------------------------------
        } else {
            let incoming_price_update = io_helpers::price_update(request.split_whitespace());
            //-------------Update vertices and edges ----------------------------
            //-------------VERTICES----------------------------------------------
            let mut vert_vec: Vec<Vertex> = Vec::new();


            //-----------------NODES-----------------------------------------------
            let vertex_source = Vertex{ exchange: incoming_price_update.exchange.clone(), currency: incoming_price_update.source_currency.clone()};
            let vertex_destination = Vertex { exchange: incoming_price_update.exchange.clone(), currency:incoming_price_update.destination_currency.clone() };

            match vertex_data.contains(&vertex_source) {
                true => {}
                false => {
                    let node_str = format!("{} {}", &vertex_source.exchange, &vertex_source.currency);
                    println!("{}", &node_str);
                    let v_source = graph.add_node(node_str);
                    vertex_data.push(vertex_source.clone());
                    vertex_index.push(v_source);
                    println!("VINDEX BUILT IN: {}",graph.index(v_source));

                }
            }
            match vertex_data.contains(&vertex_destination) {
                true => {}
                false => {
                    let node_str = format!("{} {}", &vertex_destination.exchange, &vertex_destination.currency);
                    println!("{}", &node_str);
                    let v_destination_index = graph.add_node(node_str);
                    vertex_data.push(vertex_destination.clone());
                    vertex_index.push(v_destination_index);
                }
            }
//            let source_ind = get_index_from_vertex(&vertex_source, &vertex_data, &vertex_index);

            let source_ind = vertex_data.iter().position(|x| x.eq(&vertex_source));
            match source_ind {
                None => { println!("NOT FOUND") },
                Some(_0) => { println!("Found source at index: {}", source_ind.unwrap()) }
            }

            let dest_ind = vertex_data.iter().position(|x| x.eq(&vertex_destination));
            match dest_ind {
                None =>{println!("NOT FOUND")},
                Some(_0) =>{println!("Found dest at index: {}",dest_ind.unwrap())}
            }

            // Edge adding.
            for i in &vertex_data {
                if i.currency == vertex_destination.currency {
                    // if edge does not exist.
                    println!("dest ADD EDGE HERE {} to {}", i.exchange, vertex_destination.exchange);
                    // find node index
                    let x = vertex_data.iter().position(|x| x.eq(i));
                    let y = vertex_data.iter().position(|y| y.eq(&vertex_destination));
                    let res = graph.find_edge(node_index(x.unwrap()),node_index(y.unwrap()));
                    match res {
                        None => {
                            println!("NO EDGE WAS FOUND BETWEEN {} AND {}.", i.exchange, vertex_destination.exchange);
                            graph.add_edge(node_index(x.unwrap()),node_index(y.unwrap()), 1.0);
                            edge_data.push(Edge{rate: 1.0, timestamp: incoming_price_update.timestamp});

                        },
                        Some(_0) => println!("FOUND EDGE OF INDEX: {:?}",res.unwrap())
                    }
                    let res = graph.find_edge(node_index(y.unwrap()),node_index(x.unwrap()));
                    match res {
                        None => {
                            println!("NO EDGE WAS FOUND BETWEEN {} AND {}.", i.exchange, vertex_destination.exchange);
                            graph.add_edge(node_index(y.unwrap()),node_index(x.unwrap()), 1.0);
                            edge_data.push(Edge{rate: 1.0, timestamp: incoming_price_update.timestamp});

                        },
                        Some(_0) => println!("FOUND EDGE OF INDEX: {:?}",res.unwrap())
                    }
                }
            }

            for i in &vertex_data {
                if i.currency == vertex_source.currency {
                    // if edge does not exist.
                    println!("source ADD EDGE HERE {} to {}", i.exchange, vertex_source.exchange);
                    // find node index
                    let x = vertex_data.iter().position(|x| x.eq(i));
                    let y = vertex_data.iter().position(|y| y.eq(&vertex_source));
                    let res = graph.find_edge(node_index(x.unwrap()),node_index(y.unwrap()));
                    match res {
                        None => {
                            println!("NO EDGE WAS FOUND BETWEEN {} AND {}.", i.exchange, vertex_source.exchange);
                            graph.add_edge(node_index(x.unwrap()),node_index(y.unwrap()), 1.0);
                            edge_data.push(Edge{rate: 1.0, timestamp: incoming_price_update.timestamp});

                        },
                        Some(_0) => println!("FOUND EDGE OF INDEX: {:?}",res.unwrap())
                    }
                    let res = graph.find_edge(node_index(y.unwrap()),node_index(x.unwrap()));
                    match res {
                        None => {
                            println!("NO EDGE WAS FOUND BETWEEN {} AND {}.", i.exchange, vertex_source.exchange);
                            graph.add_edge(node_index(y.unwrap()),node_index(x.unwrap()), 1.0);
                            edge_data.push(Edge{rate: 1.0, timestamp: incoming_price_update.timestamp});

                        },
                        Some(_0) => println!("FOUND EDGE OF INDEX: {:?}",res.unwrap())
                    }
                }
            }



            // ==================EDGES==============
            let source_node = get_index_from_vertex(&vertex_source, &vertex_data, &vertex_index).unwrap();

//            let source_node = vertex_index[source_ind.unwrap()];
            let dest_node = vertex_index[dest_ind.unwrap()];
            let edge_forward = Edge{rate: incoming_price_update.forward_factor, timestamp: incoming_price_update.timestamp};
            let edge_backward = Edge{rate: incoming_price_update.backward_factor, timestamp: incoming_price_update.timestamp};
            //check if edge between source and dest exists
            let edge_i = graph.find_edge(source_node, dest_node);

            match edge_i {
                None => {
                    // If not, simply add new edge.
                    let e = graph.add_edge(source_node,
                                           dest_node, edge_forward.rate);

                    edge_index.push(e);
                    edge_data.push(edge_forward);

                }
                Some(_0) => {
                    // if one exists, only update if the new rate is more recent.
                    println!("EDGES 184: {}", graph.edge_count());
                    println!("EDGE INDEX AT 184: {:?}", edge_i.unwrap());
                    if datetime_helpers::is_more_recent(incoming_price_update.timestamp,
                                                        edge_data[edge_i.unwrap().index()].timestamp ){
                        let new_edge = Edge{ rate: incoming_price_update.forward_factor, timestamp: incoming_price_update.timestamp };
                        graph.update_edge(vertex_index[source_ind.unwrap()], vertex_index[dest_ind.unwrap()], new_edge.rate);
                        edge_data[edge_i.unwrap().index()] = new_edge;

                    } else {
                        // do nothing.
                    }

                }
            }

            // Check reverse direction.
            let edge_i = graph.find_edge(dest_node, source_node);
            match edge_i {
                None => {
                    // If not, simply add new edge.
                    let e = graph.add_edge(dest_node,
                                           source_node, edge_backward.rate);

                    edge_index.push(e);
                    edge_data.push(edge_backward);

                }
                Some(_0) => {
                    // if one exists, only update if the new rate is more recent.
                    if datetime_helpers::is_more_recent(incoming_price_update.timestamp,
                                                        edge_data[edge_i.unwrap().index()].timestamp ){
                        let new_edge = Edge{ rate: incoming_price_update.backward_factor, timestamp: incoming_price_update.timestamp };
                        graph.update_edge(vertex_index[dest_ind.unwrap()], vertex_index[source_ind.unwrap()], new_edge.rate);
                        edge_data[edge_i.unwrap().index()] = new_edge;

                    } else {
                        // do nothing.
                    }

                }
            }




            println!("EDGES: {}", graph.edge_count());
            let path = algo::astar(
                &graph,
                vertex_index[0],
                |n| n==vertex_index[vertex_index.len()-1],
                |e| *e.weight(),
                |_| (0.0),
            );
            match path.clone(){
                Some((cost, path)) => {
                    println!("COST: {}, PATH: {:?}", cost, path);
                }
                None => println!("NO PATH")
            }



        }
        let mut rate: Compressed<f32> = Compressed::zero((graph.node_count(), graph.node_count()));
//        let mut rate: Conventional<f32> = Conventional::zero((graph.node_count(), graph.node_count()));


        //Builds Rate lookup table.
        for i in &vertex_index {
            for j in &vertex_index {
                let x = graph.find_edge(*i, *j);
                match x {
                    None => {print!("[{},{}] ", i.index(),j.index());
                        rate.set((i.index(),j.index()), 0.0);
                     },
                    Some(_0) => {let y = graph.edge_weight(x.unwrap());
                        print!("{:?} ", y);
                        rate.set((i.index(), j.index()), *y.unwrap());
                    }
                }
            }
            print!("\n");
            io::stdout().flush().unwrap();

        }

//        rate.set((0, 0), 42.0);
//        let mut rate = Compressed::zero((graph.node_count(), graph.node_count()));
//        let dense = Conventional::from(&sparse);


    }
}

