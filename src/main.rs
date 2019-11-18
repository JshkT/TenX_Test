/* Written by Joshua Tan in 2019
* For TenX Technical Exercise
* (The Exchange Rate Path Problem)
*/

extern crate chrono;
extern crate matrix;
extern crate petgraph;
extern crate rust_decimal;

use crate::graph_helpers::{get_index_from_vertex, get_path, vertex_string_format};
use chrono::{DateTime, FixedOffset};
use matrix::format::compressed::Variant;
use matrix::prelude::*;
use petgraph::graph::node_index;
use petgraph::prelude::NodeIndex;
use petgraph::stable_graph::EdgeIndex;
use petgraph::Graph;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use std::io;
use std::io::BufRead;

mod datetime_helpers;
mod graph_helpers;
mod io_helpers;

pub struct PriceUpdate {
    timestamp: DateTime<FixedOffset>,
    exchange: String,
    source_currency: String,
    destination_currency: String,
    forward_factor: f32,
    backward_factor: f32,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Vertex {
    exchange: String,
    currency: String,
}

#[derive(Clone, PartialEq)]
pub struct Edge {
    rate: f32,
    timestamp: DateTime<FixedOffset>,
}

pub struct ExchangeRateRequest {
    source_exchange: String,
    source_currency: String,
    destination_exchange: String,
    destination_currency: String,
}
const REQUEST_PARAMETERS: usize = 5;
const UPDATE_PARAMETERS: usize = 6;
const REQUEST_HEADER: &str = "EXCHANGE_RATE_REQUEST";
const DEBUG: bool = false;

fn main() {
    println!("Please enter either a Price Update or an Exchange Rate Request: ");

    let stdin = io::stdin();

    let mut graph = Graph::<String, f32>::new();
    let mut vertex_data: Vec<Vertex> = Vec::new();
    let mut vertex_index: Vec<NodeIndex> = Vec::new();

    let mut edge_index: Vec<EdgeIndex> = Vec::new();
    let mut edge_data: Vec<Edge> = Vec::new();

    for line in stdin.lock().lines() {
        let input_string = match &line {
            Ok(s) => s,
            Err(e) => panic!("Error getting lines from input: {}", e),
        };
        match &input_string.split_whitespace().count() {
            // Proceed only if the input matches the number of expected parameters.
            &REQUEST_PARAMETERS | &UPDATE_PARAMETERS => {
                // Check if incoming line is a Request or a Price Update
                //                let is_request = io_helpers::is_request(input_string.to_string());
                let is_request = input_string.contains(REQUEST_HEADER);
                if !is_request {
                    // Input was found to be a Price Update not a Exchange Rate Request.
                    let incoming_price_update =
                        io_helpers::price_update(input_string.split_whitespace());

                    //=============== Adding Vertices =======================================
                    let vertex_source = Vertex {
                        exchange: incoming_price_update.exchange.clone(),
                        currency: incoming_price_update.source_currency.clone(),
                    };
                    let vertex_destination = Vertex {
                        exchange: incoming_price_update.exchange.clone(),
                        currency: incoming_price_update.destination_currency.clone(),
                    };

                    // Check if source vertex exists in our data and only add if it does not.
                    match vertex_data.contains(&vertex_source) {
                        true => {}
                        false => {
                            let node_str = vertex_string_format(&vertex_source);
                            let v_source_index = graph.add_node(node_str);
                            vertex_data.push(vertex_source.clone());
                            vertex_index.push(v_source_index);
                        }
                    }
                    // Similarly for the destination vertex.
                    match vertex_data.contains(&vertex_destination) {
                        true => {}
                        false => {
                            let node_str = vertex_string_format(&vertex_destination);
                            let v_destination_index = graph.add_node(node_str);
                            vertex_data.push(vertex_destination.clone());
                            vertex_index.push(v_destination_index);
                        }
                    }

                    // ============ Time to add edges =====================
                    /* The following handles edge creation between vertexes that share
                     * the same currency but different exchanges.
                     */
                    for i in &vertex_data {
                        if i.currency == vertex_destination.currency {
                            // if edge does not exist.
                            //                            println!("dest ADD EDGE HERE {} to {}", i.exchange, vertex_destination.exchange);
                            // find node index
                            let x = vertex_data.iter().position(|x| x.eq(i));
                            let y = vertex_data.iter().position(|y| y.eq(&vertex_destination));
                            let res =
                                graph.find_edge(node_index(x.unwrap()), node_index(y.unwrap()));
                            match res {
                                None => {
                                    graph.add_edge(
                                        node_index(x.unwrap()),
                                        node_index(y.unwrap()),
                                        1.0,
                                    );
                                    edge_data.push(Edge {
                                        rate: 1.0,
                                        timestamp: incoming_price_update.timestamp,
                                    });
                                    if DEBUG {
                                        println!(
                                            "NO EDGE WAS FOUND BETWEEN {} AND {}.",
                                            i.exchange, vertex_destination.exchange
                                        );
                                    }
                                }
                                Some(_0) => {
                                    if DEBUG {
                                        println!("FOUND EDGE OF INDEX: {:?}", res.unwrap());
                                    }
                                }
                            }
                            let res =
                                graph.find_edge(node_index(y.unwrap()), node_index(x.unwrap()));
                            match res {
                                None => {
                                    graph.add_edge(
                                        node_index(y.unwrap()),
                                        node_index(x.unwrap()),
                                        1.0,
                                    );
                                    edge_data.push(Edge {
                                        rate: 1.0,
                                        timestamp: incoming_price_update.timestamp,
                                    });
                                    if DEBUG {
                                        println!(
                                            "NO EDGE WAS FOUND BETWEEN {} AND {}.",
                                            i.exchange, vertex_destination.exchange
                                        );
                                    }
                                }
                                Some(_0) => {
                                    if DEBUG {
                                        println!("FOUND EDGE OF INDEX: {:?}", res.unwrap());
                                    }
                                }
                            }
                        }
                    }
                    // Second half of checks and adding.
                    for i in &vertex_data {
                        if i.currency == vertex_source.currency {
                            // if edge does not exist.
                            // find node index
                            let x = vertex_data.iter().position(|x| x.eq(i));
                            let y = vertex_data.iter().position(|y| y.eq(&vertex_source));
                            let res =
                                graph.find_edge(node_index(x.unwrap()), node_index(y.unwrap()));
                            match res {
                                None => {
                                    graph.add_edge(
                                        node_index(x.unwrap()),
                                        node_index(y.unwrap()),
                                        1.0,
                                    );
                                    edge_data.push(Edge {
                                        rate: 1.0,
                                        timestamp: incoming_price_update.timestamp,
                                    });
                                    if DEBUG {
                                        println!(
                                            "NO EDGE WAS FOUND BETWEEN {} AND {}.",
                                            i.exchange, vertex_source.exchange
                                        );
                                    }
                                }
                                Some(_0) => {
                                    if DEBUG {
                                        println!("FOUND EDGE OF INDEX: {:?}", res.unwrap())
                                    }
                                }
                            }
                            let res =
                                graph.find_edge(node_index(y.unwrap()), node_index(x.unwrap()));
                            match res {
                                None => {
                                    graph.add_edge(
                                        node_index(y.unwrap()),
                                        node_index(x.unwrap()),
                                        1.0,
                                    );
                                    edge_data.push(Edge {
                                        rate: 1.0,
                                        timestamp: incoming_price_update.timestamp,
                                    });
                                    if DEBUG {
                                        println!(
                                            "NO EDGE WAS FOUND BETWEEN {} AND {}.",
                                            i.exchange, vertex_source.exchange
                                        );
                                    }
                                }
                                Some(_0) => {
                                    if DEBUG {
                                        println!("FOUND EDGE OF INDEX: {:?}", res.unwrap());
                                    }
                                }
                            }
                        }
                    }

                    let source_ind = vertex_data.iter().position(|x| x.eq(&vertex_source));
                    let dest_ind = vertex_data.iter().position(|x| x.eq(&vertex_destination));

                    if DEBUG {
                        match source_ind {
                            None => println!("NOT FOUND"),
                            Some(_0) => println!("Found source at index: {}", source_ind.unwrap()),
                        }

                        match dest_ind {
                            None => println!("NOT FOUND"),
                            Some(_0) => println!("Found dest at index: {}", dest_ind.unwrap()),
                        }
                    }

                    /* The following adds edges as specified in the incoming price update.
                     * It only adds edges if they are either found not to exist or if the
                     * incoming price update is more recent than the existing rate.
                     */
                    let source_node =
                        get_index_from_vertex(&vertex_source, &vertex_data, &vertex_index).unwrap();
                    let dest_node =
                        get_index_from_vertex(&vertex_destination, &vertex_data, &vertex_index)
                            .unwrap();

                    let edge_forward = Edge {
                        rate: incoming_price_update.forward_factor,
                        timestamp: incoming_price_update.timestamp,
                    };
                    let edge_backward = Edge {
                        rate: incoming_price_update.backward_factor,
                        timestamp: incoming_price_update.timestamp,
                    };

                    //check if edge between source and dest exists
                    let edge_i = graph.find_edge(source_node, dest_node);
                    match edge_i {
                        None => {
                            // If not, simply add new edge.
                            let e = graph.add_edge(source_node, dest_node, edge_forward.rate);

                            edge_index.push(e);
                            edge_data.push(edge_forward);
                        }
                        Some(_0) => {
                            // if one exists, only update if the new rate is more recent.
                            if datetime_helpers::is_more_recent(
                                incoming_price_update.timestamp,
                                edge_data[edge_i.unwrap().index()].timestamp,
                            ) {
                                let new_edge = Edge {
                                    rate: incoming_price_update.forward_factor,
                                    timestamp: incoming_price_update.timestamp,
                                };
                                graph.update_edge(
                                    vertex_index[source_ind.unwrap()],
                                    vertex_index[dest_ind.unwrap()],
                                    new_edge.rate,
                                );
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
                            let e = graph.add_edge(dest_node, source_node, edge_backward.rate);

                            edge_index.push(e);
                            edge_data.push(edge_backward);
                        }
                        Some(_0) => {
                            // if one exists, only update if the new rate is more recent.
                            if datetime_helpers::is_more_recent(
                                incoming_price_update.timestamp,
                                edge_data[edge_i.unwrap().index()].timestamp,
                            ) {
                                let new_edge = Edge {
                                    rate: incoming_price_update.backward_factor,
                                    timestamp: incoming_price_update.timestamp,
                                };
                                graph.update_edge(
                                    vertex_index[dest_ind.unwrap()],
                                    vertex_index[source_ind.unwrap()],
                                    new_edge.rate,
                                );
                                edge_data[edge_i.unwrap().index()] = new_edge;
                            } else {
                                // do nothing.
                            }
                        }
                    }
                } else {
                    /* Nothing new to add
                     *  Proceed to build graph and get best path.
                     */
                }

                if DEBUG {
                    println!("EDGES: {}", graph.edge_count());
                }

                // Best rate lookup table initialisation. As detailed in the challenge brief.
                if graph.node_count() > 0 {
                    let mut rate: Compressed<f32> =
                        Compressed::zero((graph.node_count(), graph.node_count()));
                    //Builds Rate lookup table.
                    for i in &vertex_index {
                        for j in &vertex_index {
                            let x = graph.find_edge(*i, *j);
                            match x {
                                None => {
                                    rate.set((i.index(), j.index()), 0.0);
                                }
                                Some(_0) => {
                                    let y = graph.edge_weight(x.unwrap());
                                    rate.set((i.index(), j.index()), *y.unwrap());
                                }
                            }
                        }
                    }

                    // Prints out the initial "rate" lookup table.
                    if DEBUG {
                        for i in 0..rate.rows {
                            for j in 0..rate.columns {
                                print!("{} ", rate.get((i, j)));
                            }
                            print!("\n");
                        }
                    }

                    let mut next: Compressed<usize> =
                        Compressed::new((graph.node_count(), graph.node_count()), Variant::Column);
                    for i in &vertex_index {
                        for j in &vertex_index {
                            next.set((i.index(), j.index()), j.index());
                        }
                    }
                    // Prints out the initial "next" lookup table
                    if DEBUG {
                        for i in 0..next.rows {
                            for j in 0..next.columns {
                                print!("{} ", next.get((i, j)));
                            }
                            print!("\n");
                        }
                    }

                    //==============MODIFIED FLOYD-WARSHALL======================
                    for k in 0..graph.node_count() {
                        for i in 0..graph.node_count() {
                            for j in 0..graph.node_count() {
                                let u = Decimal::from_f32(rate.get((i, j))).unwrap();
                                let a = Decimal::from_f32(rate.get((i, k))).unwrap();
                                let b = Decimal::from_f32(rate.get((k, j))).unwrap();
                                let res = a.checked_mul(b).unwrap();
                                if u < res {
                                    rate.set((i, j), Decimal::to_f32(&res).unwrap());
                                    next.set((i, j), next.get((i, k)));
                                }
                            }
                        }
                    }
                    // Turn debug on to see the lookup tables.
                    if DEBUG {
                        println!("========= UPDATED NEXTS ===========");
                        for i in 0..next.rows {
                            for j in 0..next.columns {
                                print!("{} ", next.get((i, j)));
                            }
                            print!("\n");
                        }
                        println!("========= UPDATED RATES ===========");
                        for i in 0..rate.rows {
                            for j in 0..rate.columns {
                                print!("{} ", rate.get((i, j)));
                            }
                            print!("\n");
                        }
                    }

                    if is_request {
                        let rate_request =
                            io_helpers::exchange_rate_request(input_string.split_whitespace());
                        let source_vertex = Vertex {
                            exchange: rate_request.source_exchange,
                            currency: rate_request.source_currency,
                        };
                        let dest_vertex = Vertex {
                            exchange: rate_request.destination_exchange,
                            currency: rate_request.destination_currency,
                        };

                        if vertex_data.contains(&source_vertex)
                            && vertex_data.contains(&dest_vertex)
                        {
                            let source_index =
                                get_index_from_vertex(&source_vertex, &vertex_data, &vertex_index);
                            let dest_index =
                                get_index_from_vertex(&dest_vertex, &vertex_data, &vertex_index);

                            let u = source_index.unwrap().index();
                            let v = dest_index.unwrap().index();

                            let best_rate = rate.get((u, v));
                            println!(
                                "BEST_RATES_BEGIN <{}> <{}> <{}> <{}> <{}> ",
                                &source_vertex.exchange,
                                &source_vertex.currency,
                                &dest_vertex.exchange,
                                &dest_vertex.currency,
                                best_rate
                            );

                            let path = get_path(u, v, next);
                            for x in &path {
                                let exchange = &vertex_data.get(*x).unwrap().exchange;
                                let currency = &vertex_data.get(*x).unwrap().currency;
                                println!("<{}, {}>", exchange, currency);
                            }
                            println!("BEST_RATES_END");
                        } else {
                            println!("Either Source or Destination does not exist yet.");
                        }
                    }
                }
            }
            _ => eprintln!("WRONG NUMBER OF INPUTS"),
        }
    }
}
