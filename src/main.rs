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
use crate::io_helpers::{print_results_part_one, print_results_part_two};
use crate::modified_floyd_warshall_helpers::{
    display_next_table, display_rate_table, get_best_rates, get_path_from_request,
    make_best_rate_table, make_next_table, modified_floyd_warshall,
};

use chrono::{DateTime, FixedOffset};
use matrix::prelude::Compressed;
use petgraph::Graph;
use std::io;
use std::io::{BufRead, BufReader};

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
    // These two need to outlive the line input loop. As such, I've made them mutable.
    let mut graph = Graph::<String, f32>::new();
    let mut edge_data: Vec<Edge> = Vec::new();

    println!("Please enter either a Price Update or an Exchange Rate Request: ");
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let lines = reader.lines();

    for line in lines {
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
                     *  Proceed to build rate/next tables and get best path.
                     */

                    if graph.node_count() > 0 {
                        /* Initialise best rate and next tables as defined in the challenge brief.
                         * I've used helper functions for this to keep the main function clean.
                         */
                        let rate = make_best_rate_table(&graph);
                        let next = make_next_table(&graph);

                        //============== MODIFIED FLOYD-WARSHALL ======================
                        /* Run the algorithm to get the best rates and next tables for our graph
                         * and update both lookup tables with the results.
                         */
                        let res = modified_floyd_warshall(&rate, &next, &graph);
                        let rate = res.0;
                        let next = res.1;

                        // Turn debug on to see the lookup tables.
                        if DEBUG {
                            for (i, item) in graph.raw_nodes().iter().enumerate() {
                                println!("At {} is item: {:?}", i, item);
                            }
                            for (i, item) in graph.raw_edges().iter().enumerate() {
                                println!("At {} is item: {:?}", i, item);
                            }
                            println!("========= UPDATED NEXTS ===========");
                            display_next_table(&next);
                            println!("========= UPDATED RATES ===========");
                            display_rate_table(&rate);
                        }

                        /* ========== Process Request ==============
                         *  Parse the request into a struct for clarity.
                         *  Send the rate_request to our helper function get_best_rates
                         *  which willl ookup and return the best possible rate between
                         *  the desired source and destination.
                         *
                         *  We then send the rate_request to get_path_from_request to get the
                         *  path required to achieve our best rate.
                         *
                         *  The results are displayed to the user by
                         *  using the print_results functions.
                         */
                        let rate_request =
                            io_helpers::exchange_rate_request(input_string.split_whitespace());

                        let best_rate = get_best_rates(rate_request.clone(), &rate, &graph);

                        best_rate
                            .map(|best_rate| print_results_part_one(&rate_request, &best_rate));

                        let path = get_path_from_request(&rate_request, &next, &graph);
                        print_results_part_two(&path, &graph);

                        println!("BEST_RATES_END");
                    }
                } else {
                    // Input was found to be a Price Update not a Exchange Rate Request.
                    let incoming_price_update =
                        io_helpers::price_update(input_string.split_whitespace());

                    /* ======================= Adding Vertices ===============================
                     * Check if source and/or destination vertices already exist in our graph
                     * and only add one or both do not.
                     */
                    let vertex_source = Vertex {
                        exchange: incoming_price_update.exchange.clone(),
                        currency: incoming_price_update.source_currency.clone(),
                    };
                    let vertex_destination = Vertex {
                        exchange: incoming_price_update.exchange.clone(),
                        currency: incoming_price_update.destination_currency.clone(),
                    };

                    if let false = graph_contains(&vertex_source, &graph) {
                        let node_str = vertex_string_format(&vertex_source);
                        graph.add_node(node_str);
                    }
                    // Similarly for the destination vertex.
                    if let false = graph_contains(&vertex_destination, &graph) {
                        let node_str = vertex_string_format(&vertex_destination);
                        graph.add_node(node_str);
                    }

                    // ====================== Adding edges ======================
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
                            /*  Don't need to update edge_data here because
                             *  there is nothing left to pass that information to.
                             *  We will have to rebuild it when the next update comes in.
                             *  If a request comes in, it isn't used so no need to preserve it.
                             */
                        }
                        None => println!("There was a problem adding edges between nodes."),
                    }
                }
            }
            _ => {
                eprintln!("Inputs must follow the form of a Price Update or Exchange Rate Request")
            }
        }
    }
}
