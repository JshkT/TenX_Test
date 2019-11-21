/* Written by Joshua Tan in 2019
* For TenX Technical Exercise
* (The Exchange Rate Path Problem)
*/

use crate::datetime_helpers::is_more_recent;
use crate::{Edge, PriceUpdate, Vertex, DEBUG, DEFAULT_EDGE_WEIGHT};
use chrono::{DateTime, FixedOffset};
use petgraph::graph::node_index;
use petgraph::Graph;

pub fn get_index_from_node(v: &Vertex, graph: &Graph<String, f32>) -> Option<usize> {
    for (i, item) in graph.raw_nodes().iter().enumerate() {
        if let true = item.weight.contains(&vertex_string_format(&v)) {
            return Some(i);
        }
    }
    return None;
}

/*
Formats as "<exchange> <currency>" to be used as vertex weights.
Doing this mainly to make the graph easier to keep track of.
*/
pub fn vertex_string_format(v: &Vertex) -> String {
    let node_str = format!("{}, {}", &v.exchange, &v.currency);
    if DEBUG {
        println!("{}", &node_str);
    }
    return node_str;
}

pub fn graph_contains(v: &Vertex, g: &Graph<String, f32>) -> bool {
    for item in g.raw_nodes() {
        if item.weight.eq(&vertex_string_format(v)) {
            return true;
        }
    }
    return false;
}

pub fn process_edges_same_currency(
    vertex: &Vertex,
    edge_data: &Vec<Edge>,
    incoming_price_update: &PriceUpdate,
    graph: &Graph<String, f32>,
) -> (Graph<String, f32>, Vec<Edge>) {
    let mut edge_data = edge_data.clone();
    let mut graph: Graph<String, f32> = graph.clone();

    // new_edges exists because we can't update graph until we're finished iterating over it.
    let mut new_edges: Vec<(usize, usize, f32, DateTime<FixedOffset>)> = Vec::new();

    for (index, node) in graph.raw_nodes().iter().enumerate() {
        if node.weight.contains(&vertex.currency) {
            let dest_index = get_index_from_node(&vertex, &graph);

            let edge_to_dest =
                dest_index.and_then(|i| graph.find_edge(node_index(index), node_index(i)));

            match edge_to_dest {
                None => {
                    dest_index.map(|dest_index| {
                        new_edges.push((
                            index,
                            dest_index,
                            DEFAULT_EDGE_WEIGHT,
                            incoming_price_update.timestamp,
                        ))
                    });
                    edge_data.push(Edge {
                        source_index: index,
                        dest_index: dest_index.unwrap(),
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
                    dest_index.map(|dest_index| {
                        new_edges.push((
                            dest_index,
                            index,
                            DEFAULT_EDGE_WEIGHT,
                            incoming_price_update.timestamp,
                        ))
                    });

                    edge_data.push(Edge {
                        source_index: dest_index.unwrap(),
                        dest_index: index,
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
    for e in &new_edges {
        graph.update_edge(node_index(e.0), node_index(e.1), e.2);
    }

    return (graph, edge_data);
}

pub fn process_edges_between_two_nodes(
    source_node_index: usize,
    dest_node_index: usize,
    incoming_price_update: &PriceUpdate,
    edge_data: &Vec<Edge>,
    graph: &Graph<String, f32>,
) -> (Graph<String, f32>, Vec<Edge>) {
    let mut graph = graph.clone();
    let mut edge_data = edge_data.clone();

    let edge_forward = Edge {
        source_index: source_node_index,
        dest_index: dest_node_index,
        rate: incoming_price_update.forward_factor,
        timestamp: incoming_price_update.timestamp,
    };
    let edge_backward = Edge {
        source_index: source_node_index,
        dest_index: dest_node_index,
        rate: incoming_price_update.backward_factor,
        timestamp: incoming_price_update.timestamp,
    };

    let edge_i = graph.find_edge(node_index(source_node_index), node_index(dest_node_index));
    match edge_i {
        None => {
            graph.update_edge(
                node_index(source_node_index),
                node_index(dest_node_index),
                incoming_price_update.forward_factor,
            );
            edge_data.push(edge_forward);
        }
        Some(e_curr) => {
            // if one exists, only update if the new rate is more recent.
            if is_more_recent(
                incoming_price_update.timestamp,
                edge_data[e_curr.index()].timestamp,
            ) {
                graph.update_edge(
                    node_index(source_node_index),
                    node_index(dest_node_index),
                    edge_forward.rate,
                );
                edge_data[e_curr.index()] = edge_forward;
            } else {
                // Don't update if new update isn't more recent.
            }
        }
    }

    // Reverse destination and source to find backward edge.
    let edge_i = graph.find_edge(node_index(dest_node_index), node_index(source_node_index));
    match edge_i {
        None => {
            graph.update_edge(
                node_index(dest_node_index),
                node_index(source_node_index),
                incoming_price_update.backward_factor,
            );
            edge_data.push(edge_backward);
        }
        Some(e_curr) => {
            // if one exists, only update if the new rate is more recent.
            if is_more_recent(
                incoming_price_update.timestamp,
                edge_data[e_curr.index()].timestamp,
            ) {
                graph.update_edge(
                    node_index(dest_node_index),
                    node_index(source_node_index),
                    edge_backward.rate,
                );
                edge_data[e_curr.index()] = edge_backward;
            } else {
                // Don't update if new update isn't more recent.
            }
        }
    }
    return (graph, edge_data);
}
