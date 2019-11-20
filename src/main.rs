/* Written by Joshua Tan in 2019
* For TenX Technical Exercise
* (The Exchange Rate Path Problem)
*/

extern crate chrono;
extern crate matrix;
extern crate petgraph;
extern crate rust_decimal;

use crate::graph_helpers::{
    get_best_rates, get_index_from_node, get_path_from_request, graph_contains,
    make_best_rate_table, make_next_table, modified_floyd_warshall, vertex_string_format,
};

use chrono::{DateTime, FixedOffset};
use petgraph::graph::node_index;
use petgraph::prelude::NodeIndex;
use petgraph::stable_graph::EdgeIndex;
use petgraph::Graph;
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
#[derive(Clone, Eq, PartialEq)]
pub struct ExchangeRateRequest {
    source_exchange: String,
    source_currency: String,
    destination_exchange: String,
    destination_currency: String,
}
const REQUEST_PARAMETERS: usize = 5;
const UPDATE_PARAMETERS: usize = 6;
const REQUEST_HEADER: &str = "EXCHANGE_RATE_REQUEST";
const DEFAULT_EDGE_WEIGHT: f32 = 1.0;
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
                    if let false = vertex_data.contains(&vertex_source) {
                        let node_str = vertex_string_format(&vertex_source);
                        let v_source_index = graph.add_node(node_str);
                        vertex_data.push(vertex_source.clone());
                        vertex_index.push(v_source_index);
                    }
                    // Similarly for the destination vertex.
                    if let false = vertex_data.contains(&vertex_destination) {
                        let node_str = vertex_string_format(&vertex_destination);
                        let v_destination_index = graph.add_node(node_str);
                        vertex_data.push(vertex_destination.clone());
                        vertex_index.push(v_destination_index);
                    }

                    if let false = graph_contains(&vertex_source, &graph) {
                        let node_str = vertex_string_format(&vertex_source);
                        graph.add_node(node_str);
                    }

                    if let false = graph_contains(&vertex_destination, &graph) {
                        let node_str = vertex_string_format(&vertex_destination);
                        graph.add_node(node_str);
                    }

                    // ============ Time to add edges =====================
                    /* The following handles edge creation between vertexes that share
                     * the same currency but different exchanges.
                     */

                    //                    let res = process_edges(
                    //                        &vertex_destination,
                    //                        &edge_data,
                    //                        &incoming_price_update,
                    //                        graph,
                    //                    );
                    //                    let mut graph = res.0.clone();
                    //                    let mut edge_data = res.1;
                    for i in &vertex_data {
                        if i.currency == vertex_destination.currency {
                            // if edge does not exist.
                            //                            println!("dest ADD EDGE HERE {} to {}", i.exchange, vertex_destination.exchange);
                            // find node index
                            let x = vertex_data.iter().position(|x| x.eq(i));
                            let y = vertex_data.iter().position(|y| y.eq(&vertex_destination));

                            let x1 = x.map(|n| node_index(n));
                            let y1 = y.map(|n| node_index(n));
                            let res = x1.and_then(|x2| y1.and_then(|y2| graph.find_edge(x2, y2)));

                            match res {
                                None => {
                                    graph.update_edge(
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
                            let res = x1.and_then(|x2| y1.and_then(|y2| graph.find_edge(y2, x2)));
                            match res {
                                None => {
                                    graph.update_edge(
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
                                    graph.update_edge(
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
                                    graph.update_edge(
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

                    let source_ind = get_index_from_node(&vertex_source, &graph);
                    //                    let source_ind = vertex_data.iter().position(|x| x.eq(&vertex_source));
                    let dest_ind = get_index_from_node(&vertex_destination, &graph);
                    //                    let dest_ind = vertex_data.iter().position(|x| x.eq(&vertex_destination));

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
                    //                    let source_node =
                    //                        get_index_from_vertex(&vertex_source, &vertex_data, &vertex_index);
                    let source_node = get_index_from_node(&vertex_source, &graph);
                    //                    let dest_node =
                    //                        get_index_from_vertex(&vertex_destination, &vertex_data, &vertex_index);
                    let dest_node = get_index_from_node(&vertex_destination, &graph);

                    let edge_forward = Edge {
                        rate: incoming_price_update.forward_factor,
                        timestamp: incoming_price_update.timestamp,
                    };
                    let edge_backward = Edge {
                        rate: incoming_price_update.backward_factor,
                        timestamp: incoming_price_update.timestamp,
                    };

                    //check if edge between source and dest exists
                    //                    let edge_i = graph.find_edge(source_node, dest_node);
                    let edge_i = source_node.and_then(|u| {
                        dest_node.and_then(|v| graph.find_edge(node_index(u), node_index(v)))
                    });
                    match edge_i {
                        None => {
                            // If not, simply add new edge.

                            let e = source_node.and_then(|u| {
                                dest_node.map(|v| {
                                    graph.update_edge(
                                        node_index(u),
                                        node_index(v),
                                        edge_forward.rate,
                                    )
                                })
                            });

                            e.map(|e| edge_index.push(e));
                            //                            edge_index.push(e);
                            edge_data.push(edge_forward);
                        }
                        Some(e_curr) => {
                            // if one exists, only update if the new rate is more recent.

                            if datetime_helpers::is_more_recent(
                                incoming_price_update.timestamp,
                                edge_data[e_curr.index()].timestamp,
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
                                edge_data[e_curr.index()] = new_edge;
                            } else {
                                // do nothing.
                            }
                        }
                    }

                    // Check reverse direction.
                    //                    let edge_i = graph.find_edge(dest_node.unwrap(), source_node.unwrap());
                    let edge_i = source_node.and_then(|u| {
                        dest_node.and_then(|v| graph.find_edge(node_index(v), node_index(u)))
                    });
                    match edge_i {
                        None => {
                            // If not, simply add new edge.
                            let e = source_node.and_then(|u| {
                                dest_node.map(|v| {
                                    graph.update_edge(
                                        node_index(v),
                                        node_index(u),
                                        edge_backward.rate,
                                    )
                                })
                            });

                            e.map(|e| edge_index.push(e));
                            //                            edge_index.push(e);
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
                }
                /* Nothing new to add
                 *  Proceed to build graph and get best path.
                 */

                if DEBUG {
                    println!("EDGES: {}", graph.edge_count());
                }

                // Best rate lookup table initialisation. As detailed in the challenge brief.
                if graph.node_count() > 0 {
                    let rate = make_best_rate_table(&graph);
                    let next = make_next_table(&graph);

                    //==============MODIFIED FLOYD-WARSHALL======================
                    let res = modified_floyd_warshall(&rate, &next, &graph);

                    // Update rate and next tables.
                    let rate = res.0;
                    let next = res.1;

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
                    for (i, item) in graph.raw_nodes().iter().enumerate() {
                        println!("At {} is item: {:?}", i, item);
                    }
                    for (i, item) in graph.raw_edges().iter().enumerate() {
                        println!("At {} is item: {:?}", i, item);
                    }

                    if is_request {
                        let rate_request =
                            io_helpers::exchange_rate_request(input_string.split_whitespace());

                        let best_rate = get_best_rates(rate_request.clone(), &rate, &graph);

                        best_rate
                            .map(|best_rate| print_results_part_one(&rate_request, &best_rate));

                        let path = get_path_from_request(&rate_request, &next, &graph);
                        print_results_part_two(&path, &graph);
                        //                        path.map(|path| print_results_part_two(path, &graph));

                        println!("BEST_RATES_END");
                    }
                }
            }
            _ => eprintln!("WRONG NUMBER OF INPUTS"),
        }
    }
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

pub fn process_edges(
    vertex: &Vertex,
    edge_data: &Vec<Edge>,
    incoming_price_update: &PriceUpdate,
    graph: Graph<String, f32>,
) -> (Graph<String, f32>, Vec<Edge>) {
    let mut edge_data = edge_data.clone();
    for (index, node) in graph.raw_nodes().iter().enumerate() {
        if node.weight.contains(&vertex.currency) {
            let dest_index = get_index_from_node(&vertex, &graph);

            let edge_to_dest =
                dest_index.and_then(|i| graph.find_edge(node_index(index), node_index(i)));

            match edge_to_dest {
                None => {
                    let mut graph = graph.clone();
                    dest_index.map(|dest_index| {
                        graph.update_edge(
                            node_index(index),
                            node_index(dest_index),
                            DEFAULT_EDGE_WEIGHT,
                        )
                    });
                    edge_data.push(Edge {
                        rate: DEFAULT_EDGE_WEIGHT,
                        timestamp: incoming_price_update.timestamp,
                    });
                }
                Some(e) => {
                    if DEBUG {
                        println!(
                            "FOUND EDGE OF INDEX: {:?}, {:?}",
                            e,
                            edge_data[e.index()].timestamp
                        );
                    }
                }
            }

            let edge_to_dest =
                dest_index.and_then(|i| graph.find_edge(node_index(i), node_index(index)));

            match edge_to_dest {
                None => {
                    let mut graph = graph.clone();
                    dest_index.map(|dest_index| {
                        graph.update_edge(
                            node_index(index),
                            node_index(dest_index),
                            DEFAULT_EDGE_WEIGHT,
                        )
                    });
                    edge_data.push(Edge {
                        rate: DEFAULT_EDGE_WEIGHT,
                        timestamp: incoming_price_update.timestamp,
                    });
                }
                Some(e) => {
                    if DEBUG {
                        println!(
                            "FOUND EDGE OF INDEX: {:?}, {:?}",
                            e,
                            edge_data[e.index()].timestamp
                        );
                    }
                }
            }
        }
    }
    return (graph, edge_data);
}
