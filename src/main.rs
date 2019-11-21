/* Written by Joshua Tan in 2019
* For TenX Technical Exercise
* (The Exchange Rate Path Problem)
*/

extern crate chrono;
extern crate matrix;
extern crate petgraph;
extern crate rust_decimal;

use crate::graph_helpers::{
    get_index_from_node, graph_contains, process_edges_between_two_nodes,
    process_edges_same_currency, vertex_string_format,
};
use crate::modified_floyd_warshall_helpers::{
    get_best_rates, get_path_from_request, make_best_rate_table, make_next_table,
    modified_floyd_warshall,
};

use chrono::{DateTime, FixedOffset};
use petgraph::graph::node_index;
use petgraph::prelude::NodeIndex;
use petgraph::Graph;
use std::io;
use std::io::BufRead;

mod datetime_helpers;
mod graph_helpers;
mod io_helpers;
mod modified_floyd_warshall_helpers;

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

#[derive(Clone, PartialEq, Debug)]
pub struct Edge {
    source_index: usize,
    dest_index: usize,
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

                if is_request {
                    /* If incoming line is a request, Nothing new to add
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
                            println!("{:?}", rate);

                            let best_rate = get_best_rates(rate_request.clone(), &rate, &graph);

                            best_rate
                                .map(|best_rate| print_results_part_one(&rate_request, &best_rate));

                            let path = get_path_from_request(&rate_request, &next, &graph);
                            print_results_part_two(&path, &graph);
                            //                        path.map(|path| print_results_part_two(path, &graph));

                            println!("BEST_RATES_END");
                        }
                    }
                } else {
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
                     *
                     *  Check if vertex_destination can connect to other exchanges
                     *  that support that currency. Ignores if edge between those already exists.
                     *  adds a new edge of weight between them if it does not exist.
                     *  This includes an edge to itself with a weight of 1.0.
                     */
                    let res = process_edges_same_currency(
                        &vertex_destination,
                        &edge_data,
                        &incoming_price_update,
                        &graph,
                    );

                    // Update graph and edge_data with our results.
                    graph = res.0;
                    edge_data = res.1;

                    /* Do the same for vertex_source's currency.
                     */
                    let res = process_edges_same_currency(
                        &vertex_source,
                        &edge_data,
                        &incoming_price_update,
                        &graph,
                    );

                    // Update graph and edge_data with our results.
                    graph = res.0;
                    edge_data = res.1;

                    /* The following adds edges as specified in the incoming price update.
                     * It only adds edges if they are either found not to exist or if the
                     * incoming price update is more recent than the existing rate.
                     * This is done through the function process_edges_between_two_nodes.
                     */
                    let source_node_index = get_index_from_node(&vertex_source, &graph);
                    let dest_node_index = get_index_from_node(&vertex_destination, &graph);

                    let res = source_node_index.and_then(|source_node_index| {
                        dest_node_index.map(|dest_node_index| {
                            process_edges_between_two_nodes(
                                source_node_index,
                                dest_node_index,
                                &incoming_price_update,
                                &edge_data,
                                &graph,
                            )
                        })
                    });
                    match res {
                        Some(r) => {
                            graph = r.0;
                            edge_data = r.1;
                        }
                        None => println!("There was a problem adding edges between nodes."),
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
